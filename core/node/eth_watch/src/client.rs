use std::fmt;

use anyhow::Context;
use zksync_contracts::{state_transition_manager_contract, verifier_contract};
use zksync_eth_client::{
    clients::{DynClient, L1},
    CallFunctionArgs, ClientError, ContractCallError, EnrichedClientError, EnrichedClientResult,
    EthInterface,
};
use zksync_types::{
    ethabi::Contract,
    web3::{BlockId, BlockNumber, FilterBuilder, Log},
    Address, H256,
};

/// L1 client functionality used by [`EthWatch`](crate::EthWatch) and constituent event processors.
#[async_trait::async_trait]
pub trait EthClient: 'static + fmt::Debug + Send + Sync {
    /// Returns events in a given block range.
    async fn get_events(
        &self,
        from: BlockNumber,
        to: BlockNumber,
        retries_left: usize,
    ) -> EnrichedClientResult<Vec<Log>>;
    /// Returns finalized L1 block number.
    async fn finalized_block_number(&self) -> EnrichedClientResult<u64>;
    /// Returns scheduler verification key hash by verifier address.
    async fn scheduler_vk_hash(&self, verifier_address: Address)
        -> Result<H256, ContractCallError>;
    /// Returns upgrade diamond cut by packed protocol version.
    async fn diamond_cut_by_version(
        &self,
        packed_version: H256,
    ) -> EnrichedClientResult<Option<Vec<u8>>>;
    /// Sets list of topics to return events for.
    fn set_topics(&mut self, topics: Vec<H256>);
}

pub const RETRY_LIMIT: usize = 5;
const TOO_MANY_RESULTS_INFURA: &str = "query returned more than";
const TOO_MANY_RESULTS_ALCHEMY: &str = "response size exceeded";
const TOO_MANY_RESULTS_RETH: &str = "query exceeds max block range";

/// Implementation of [`EthClient`] based on HTTP JSON-RPC (encapsulated via [`EthInterface`]).
#[derive(Debug)]
pub struct EthHttpQueryClient {
    client: Box<DynClient<L1>>,
    topics: Vec<H256>,
    diamond_proxy_addr: Address,
    governance_address: Address,
    new_upgrade_cut_data_signature: H256,
    // Only present for post-shared bridge chains.
    state_transition_manager_address: Option<Address>,
    chain_admin_address: Option<Address>,
    verifier_contract_abi: Contract,
    confirmations_for_eth_event: Option<u64>,
}

impl EthHttpQueryClient {
    pub fn new(
        client: Box<DynClient<L1>>,
        diamond_proxy_addr: Address,
        state_transition_manager_address: Option<Address>,
        chain_admin_address: Option<Address>,
        governance_address: Address,
        confirmations_for_eth_event: Option<u64>,
    ) -> Self {
        tracing::debug!(
            "New eth client, ZKsync addr: {:x}, governance addr: {:?}",
            diamond_proxy_addr,
            governance_address
        );
        Self {
            client: client.for_component("watch"),
            topics: Vec::new(),
            diamond_proxy_addr,
            state_transition_manager_address,
            chain_admin_address,
            governance_address,
            new_upgrade_cut_data_signature: state_transition_manager_contract()
                .event("NewUpgradeCutData")
                .context("NewUpgradeCutData event is missing in ABI")
                .unwrap()
                .signature(),
            verifier_contract_abi: verifier_contract(),
            confirmations_for_eth_event,
        }
    }

    fn get_default_address_list(&self) -> Vec<Address> {
        [
            Some(self.diamond_proxy_addr),
            Some(self.governance_address),
            self.state_transition_manager_address,
            self.chain_admin_address,
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    async fn get_events_inner(
        &self,
        from: BlockNumber,
        to: BlockNumber,
        topics1: Vec<H256>,
        topics2: Vec<H256>,
        addresses: Vec<Address>,
        retries_left: usize,
    ) -> EnrichedClientResult<Vec<Log>> {
        let filter = FilterBuilder::default()
            .from_block(from)
            .to_block(to)
            .topics(Some(topics1), Some(topics2), None, None)
            .address(addresses)
            .build();
        let mut result = self.client.logs(&filter).await;

        // This code is compatible with both Infura and Alchemy API providers.
        // Note: we don't handle rate-limits here - assumption is that we're never going to hit them.
        if let Err(err) = &result {
            tracing::warn!("Provider returned error message: {err}");
            let err_message = err.as_ref().to_string();
            let err_code = if let ClientError::Call(err) = err.as_ref() {
                Some(err.code())
            } else {
                None
            };

            let should_retry = |err_code, err_message: String| {
                // All of these can be emitted by either API provider.
                err_code == Some(-32603)             // Internal error
                    || err_message.contains("failed")    // Server error
                    || err_message.contains("timed out") // Time-out error
            };

            // check whether the error is related to having too many results
            if err_message.contains(TOO_MANY_RESULTS_INFURA)
                || err_message.contains(TOO_MANY_RESULTS_ALCHEMY)
                || err_message.contains(TOO_MANY_RESULTS_RETH)
            {
                // get the numeric block ids
                let from_number = match from {
                    BlockNumber::Number(num) => num,
                    _ => {
                        // invalid variant
                        return result;
                    }
                };
                let to_number = match to {
                    BlockNumber::Number(num) => num,
                    BlockNumber::Latest => self.client.block_number().await?,
                    _ => {
                        // invalid variant
                        return result;
                    }
                };

                // divide range into two halves and recursively fetch them
                let mid = (from_number + to_number) / 2;

                // safety check to prevent infinite recursion (quite unlikely)
                if from_number >= mid {
                    tracing::warn!("Infinite recursion detected while getting events: from_number={from_number:?}, mid={mid:?}");
                    return result;
                }

                tracing::warn!("Splitting block range in half: {from:?} - {mid:?} - {to:?}");
                let mut first_half = self
                    .get_events(from, BlockNumber::Number(mid), RETRY_LIMIT)
                    .await?;
                let mut second_half = self
                    .get_events(BlockNumber::Number(mid + 1u64), to, RETRY_LIMIT)
                    .await?;

                first_half.append(&mut second_half);
                result = Ok(first_half);
            } else if should_retry(err_code, err_message) && retries_left > 0 {
                tracing::warn!("Retrying. Retries left: {retries_left}");
                result = self.get_events(from, to, retries_left - 1).await;
            }
        }

        result
    }
}

#[async_trait::async_trait]
impl EthClient for EthHttpQueryClient {
    async fn scheduler_vk_hash(
        &self,
        verifier_address: Address,
    ) -> Result<H256, ContractCallError> {
        // New verifier returns the hash of the verification key.
        CallFunctionArgs::new("verificationKeyHash", ())
            .for_contract(verifier_address, &self.verifier_contract_abi)
            .call(&self.client)
            .await
    }

    async fn diamond_cut_by_version(
        &self,
        packed_version: H256,
    ) -> EnrichedClientResult<Option<Vec<u8>>> {
        const LOOK_BACK_BLOCK_RANGE: u64 = 1_000_000;

        let Some(state_transition_manager_address) = self.state_transition_manager_address else {
            return Ok(None);
        };

        let to_block = self.client.block_number().await?;
        let from_block = to_block.saturating_sub((LOOK_BACK_BLOCK_RANGE - 1).into());

        let logs = self
            .get_events_inner(
                from_block.into(),
                to_block.into(),
                vec![self.new_upgrade_cut_data_signature],
                vec![packed_version],
                vec![state_transition_manager_address],
                RETRY_LIMIT,
            )
            .await?;

        Ok(logs.into_iter().next().map(|log| log.data.0))
    }

    async fn get_events(
        &self,
        from: BlockNumber,
        to: BlockNumber,
        retries_left: usize,
    ) -> EnrichedClientResult<Vec<Log>> {
        self.get_events_inner(
            from,
            to,
            self.topics.clone(),
            Vec::new(),
            self.get_default_address_list(),
            retries_left,
        )
        .await
    }

    async fn finalized_block_number(&self) -> EnrichedClientResult<u64> {
        if let Some(confirmations) = self.confirmations_for_eth_event {
            let latest_block_number = self.client.block_number().await?.as_u64();
            Ok(latest_block_number.saturating_sub(confirmations))
        } else {
            let block = self
                .client
                .block(BlockId::Number(BlockNumber::Finalized))
                .await?
                .ok_or_else(|| {
                    let err = ClientError::Custom("Finalized block must be present on L1".into());
                    EnrichedClientError::new(err, "block")
                })?;
            let block_number = block.number.ok_or_else(|| {
                let err = ClientError::Custom("Finalized block must contain number".into());
                EnrichedClientError::new(err, "block").with_arg("block", &block)
            })?;
            Ok(block_number.as_u64())
        }
    }

    fn set_topics(&mut self, topics: Vec<H256>) {
        self.topics = topics;
    }
}
