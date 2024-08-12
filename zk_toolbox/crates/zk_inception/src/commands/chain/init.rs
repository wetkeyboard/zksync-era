use anyhow::Context;
use common::{
    config::global_config,
    forge::{Forge, ForgeScriptArgs},
    git, logger,
    spinner::Spinner,
};
use config::{
    copy_configs,
    forge_interface::{
        register_chain::{input::RegisterChainL1Config, output::RegisterChainOutput},
        script_params::REGISTER_CHAIN_SCRIPT_PARAMS,
    },
    set_l1_rpc_url,
    traits::{ReadConfig, SaveConfig, SaveConfigWithBasePath},
    update_from_chain_config, ChainConfig, ContractsConfig, EcosystemConfig, GeneralConfig,
};
use url::Url;
use xshell::Shell;

use crate::{
    accept_ownership::accept_admin,
    commands::chain::{
        args::init::{InitArgs, InitArgsFinal},
        deploy_l2_contracts, deploy_paymaster,
        genesis::genesis,
    },
    messages::{
        msg_initializing_chain, MSG_ACCEPTING_ADMIN_SPINNER, MSG_CHAIN_INITIALIZED,
        MSG_CHAIN_NOT_FOUND_ERR, MSG_GENESIS_DATABASE_ERR, MSG_REGISTERING_CHAIN_SPINNER,
        MSG_SELECTED_CONFIG,
    },
    utils::forge::{check_the_balance, fill_forge_private_key},
};

pub(crate) async fn run(args: InitArgs, shell: &Shell) -> anyhow::Result<()> {
    let chain_name = global_config().chain_name.clone();
    let config = EcosystemConfig::from_file(shell)?;
    let chain_config = config
        .load_chain(chain_name)
        .context(MSG_CHAIN_NOT_FOUND_ERR)?;
    let mut args = args.fill_values_with_prompt(&chain_config);

    logger::note(MSG_SELECTED_CONFIG, logger::object_to_string(&chain_config));
    logger::info(msg_initializing_chain(""));
    git::submodule_update(shell, config.link_to_code.clone())?;

    init(&mut args, shell, &config, &chain_config).await?;

    logger::success(MSG_CHAIN_INITIALIZED);
    Ok(())
}

pub async fn init(
    init_args: &mut InitArgsFinal,
    shell: &Shell,
    ecosystem_config: &EcosystemConfig,
    chain_config: &ChainConfig,
) -> anyhow::Result<()> {
    copy_configs(shell, &ecosystem_config.link_to_code, &chain_config.configs)?;

    let mut general_config = chain_config.get_general_config()?;
    apply_port_offset(init_args.port_offset, &mut general_config)?;
    general_config.save_with_base_path(shell, &chain_config.configs)?;

    let mut genesis_config = chain_config.get_genesis_config()?;
    update_from_chain_config(&mut genesis_config, chain_config);
    genesis_config.save_with_base_path(shell, &chain_config.configs)?;

    // Copy ecosystem contracts
    let mut contracts_config = ecosystem_config.get_contracts_config()?;
    contracts_config.l1.base_token_addr = chain_config.base_token.address;
    contracts_config.save_with_base_path(shell, &chain_config.configs)?;

    crate::commands::ecosystem::init::distribute_eth(
        ecosystem_config,
        chain_config,
        init_args.l1_rpc_url.clone(),
    )
    .await?;
    let mut secrets = chain_config.get_secrets_config()?;
    set_l1_rpc_url(&mut secrets, init_args.l1_rpc_url.clone())?;
    secrets.save_with_base_path(shell, &chain_config.configs)?;

    let spinner = Spinner::new(MSG_REGISTERING_CHAIN_SPINNER);
    register_chain(
        shell,
        init_args.forge_args.clone(),
        ecosystem_config,
        chain_config,
        &mut contracts_config,
        init_args.l1_rpc_url.clone(),
    )
    .await?;
    contracts_config.save_with_base_path(shell, &chain_config.configs)?;
    spinner.finish();
    let spinner = Spinner::new(MSG_ACCEPTING_ADMIN_SPINNER);
    accept_admin(
        shell,
        ecosystem_config,
        contracts_config.l1.chain_admin_addr,
        chain_config.get_wallets_config()?.governor_private_key(),
        contracts_config.l1.diamond_proxy_addr,
        &init_args.forge_args.clone(),
        init_args.l1_rpc_url.clone(),
    )
    .await?;
    spinner.finish();

    deploy_l2_contracts::deploy_l2_contracts(
        shell,
        chain_config,
        ecosystem_config,
        &mut contracts_config,
        init_args.forge_args.clone(),
    )
    .await?;
    contracts_config.save_with_base_path(shell, &chain_config.configs)?;

    if init_args.deploy_paymaster {
        deploy_paymaster::deploy_paymaster(
            shell,
            chain_config,
            &mut contracts_config,
            init_args.forge_args.clone(),
        )
        .await?;
        contracts_config.save_with_base_path(shell, &chain_config.configs)?;
    }

    genesis(init_args.genesis_args.clone(), shell, chain_config)
        .await
        .context(MSG_GENESIS_DATABASE_ERR)?;

    Ok(())
}

async fn register_chain(
    shell: &Shell,
    forge_args: ForgeScriptArgs,
    config: &EcosystemConfig,
    chain_config: &ChainConfig,
    contracts: &mut ContractsConfig,
    l1_rpc_url: String,
) -> anyhow::Result<()> {
    let deploy_config_path = REGISTER_CHAIN_SCRIPT_PARAMS.input(&config.link_to_code);

    let deploy_config = RegisterChainL1Config::new(chain_config, contracts)?;
    deploy_config.save(shell, deploy_config_path)?;

    let mut forge = Forge::new(&config.path_to_foundry())
        .script(&REGISTER_CHAIN_SCRIPT_PARAMS.script(), forge_args.clone())
        .with_ffi()
        .with_rpc_url(l1_rpc_url)
        .with_broadcast();

    forge = fill_forge_private_key(forge, config.get_wallets()?.governor_private_key())?;
    check_the_balance(&forge).await?;
    forge.run(shell)?;

    let register_chain_output = RegisterChainOutput::read(
        shell,
        REGISTER_CHAIN_SCRIPT_PARAMS.output(&chain_config.link_to_code),
    )?;
    contracts.set_chain_contracts(&register_chain_output);
    Ok(())
}

fn apply_port_offset(port_offset: u16, general_config: &mut GeneralConfig) -> anyhow::Result<()> {
    if let Some(ref mut api) = general_config.api_config {
        api.web3_json_rpc.http_port += port_offset;
        api.web3_json_rpc.ws_port += port_offset;

        let mut http_url = Url::parse(&api.web3_json_rpc.http_url)?;
        let _ = http_url.set_port(http_url.port().map(|p| p + port_offset));
        api.web3_json_rpc.http_url = http_url.to_string();

        let mut ws_url = Url::parse(&api.web3_json_rpc.ws_url)?;
        let _ = ws_url.set_port(ws_url.port().map(|p| p + port_offset));
        api.web3_json_rpc.ws_url = ws_url.to_string();

        api.prometheus.listener_port += port_offset;
        api.healthcheck.port += port_offset;
        api.merkle_tree.port += port_offset;
    }

    if let Some(ref mut contract_verifier) = general_config.contract_verifier {
        contract_verifier.port += port_offset;

        let mut url = Url::parse(&contract_verifier.url)?;
        let _ = url.set_port(url.port().map(|p| p + port_offset));
        contract_verifier.url = url.to_string();
    }

    if let Some(ref mut prometheus) = general_config.prometheus_config {
        prometheus.listener_port += port_offset;
    }

    Ok(())
}
