use clap::Parser;
use common::{forge::ForgeScriptArgs, Prompt};
use config::ChainConfig;
use serde::{Deserialize, Serialize};
use types::L1Network;
use url::Url;

use super::{genesis::GenesisArgsFinal, init::PortOffset};
use crate::{
    commands::chain::args::genesis::GenesisArgs,
    defaults::LOCAL_RPC_URL,
    messages::{
        MSG_DEPLOY_PAYMASTER_PROMPT, MSG_GENESIS_ARGS_HELP, MSG_L1_RPC_URL_HELP,
        MSG_L1_RPC_URL_INVALID_ERR, MSG_L1_RPC_URL_PROMPT, MSG_PORT_OFFSET_HELP,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, Parser)]
pub struct BuildArgs {
    /// All ethereum environment related arguments
    #[clap(flatten)]
    #[serde(flatten)]
    pub forge_args: ForgeScriptArgs,
    #[clap(flatten, next_help_heading = MSG_GENESIS_ARGS_HELP)]
    #[serde(flatten)]
    pub genesis_args: GenesisArgs,
    #[clap(long, default_missing_value = "true", num_args = 0..=1)]
    pub deploy_paymaster: Option<bool>,
    #[clap(long, help = MSG_L1_RPC_URL_HELP)]
    pub l1_rpc_url: Option<String>,
    #[clap(long, help = MSG_PORT_OFFSET_HELP)]
    pub port_offset: Option<PortOffset>,
}

impl BuildArgs {
    pub fn fill_values_with_prompt(self, config: &ChainConfig) -> BuildArgsFinal {
        let deploy_paymaster = self.deploy_paymaster.unwrap_or_else(|| {
            common::PromptConfirm::new(MSG_DEPLOY_PAYMASTER_PROMPT)
                .default(true)
                .ask()
        });

        let l1_rpc_url = self.l1_rpc_url.unwrap_or_else(|| {
            let mut prompt = Prompt::new(MSG_L1_RPC_URL_PROMPT);
            if config.l1_network == L1Network::Localhost {
                prompt = prompt.default(LOCAL_RPC_URL);
            }
            prompt
                .validate_with(|val: &String| -> Result<(), String> {
                    Url::parse(val)
                        .map(|_| ())
                        .map_err(|_| MSG_L1_RPC_URL_INVALID_ERR.to_string())
                })
                .ask()
        });

        BuildArgsFinal {
            forge_args: self.forge_args,
            genesis_args: self.genesis_args.fill_values_with_prompt(config),
            deploy_paymaster,
            l1_rpc_url,
            port_offset: self
                .port_offset
                .unwrap_or(PortOffset::from_chain_id(config.chain_id.as_u64() as u16))
                .into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildArgsFinal {
    pub forge_args: ForgeScriptArgs,
    pub genesis_args: GenesisArgsFinal,
    pub deploy_paymaster: bool,
    pub l1_rpc_url: String,
    pub port_offset: u16,
}
