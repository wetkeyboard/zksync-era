/// Tool for load testing the gossipnet endpoint of the external node.
use anyhow::Context as _;
use clap::Parser;
use zksync_concurrency::{ctx, net};
use zksync_consensus_crypto::Text;
use zksync_consensus_network::gossip::loadtest;
use zksync_types::url::SensitiveUrl;
use zksync_web3_decl::{
    client::{Client, DynClient, L2},
    namespaces::EnNamespaceClient,
};

#[derive(Debug, Parser)]
struct Args {
    /// URL of the main node - used to fetch consensus genesis.
    #[arg(long)]
    main_node_url: SensitiveUrl,
    /// Address of the endpoint to loadtest (`<IP>:<port>` or `<domain>:<port>`).
    #[arg(long)]
    peer_addr: String,
    /// Node public key of the peer.
    #[arg(long)]
    peer_node_key: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = zksync_vlog::ObservabilityBuilder::new().build();

    let ctx = &ctx::root();
    let args = Args::parse();

    // Fetch genesis.
    let client: Box<DynClient<L2>> = Box::new(
        Client::http(args.main_node_url.clone())
            .context("Client::http()")?
            .build(),
    );
    let genesis = ctx
        .wait(client.consensus_genesis())
        .await?
        .context("fetch_consensus_genesis()")?
        .context("main node is not running consensus component")?;
    let genesis =
        zksync_protobuf::serde::deserialize(&genesis.0).context("deserialize(genesis)")?;
    tracing::info!("genesis = {genesis:?}");

    loadtest::Loadtest {
        addr: net::Host(args.peer_addr),
        peer: Text::new(&args.peer_node_key)
            .decode()
            .context("peer_node_key")?,
        genesis,
        traffic_pattern: loadtest::TrafficPattern::Sequential,
        output: None,
    }
    .run(ctx)
    .await?;

    Ok(())
}
