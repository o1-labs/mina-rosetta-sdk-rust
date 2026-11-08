//! The tip, the genesis block and the sync state of a rosetta server.
//!
//! ```bash
//! ROSETTA_URL=http://127.0.0.1:3087 cargo run --example network_status
//! ```

use mina_rosetta_sdk::RosettaClient;

#[tokio::main]
async fn main() -> mina_rosetta_sdk::Result<()> {
    let url = std::env::var("ROSETTA_URL").unwrap_or_else(|_| "http://127.0.0.1:3087".into());
    let client = RosettaClient::discover(&url).await?;
    println!("network {}", client.network().network);

    let status = client.network_status().await?;
    let tip = &status.current_block_identifier;
    println!("tip     {} {}", tip.index, tip.hash);
    let genesis = &status.genesis_block_identifier;
    println!("genesis {} {}", genesis.index, genesis.hash);
    if let Some(sync) = &status.sync_status {
        println!("sync    {:?} synced={:?}", sync.stage, sync.synced);
    }
    println!("peers   {}", status.peers.len());

    let options = client.network_options().await?;
    println!("node    {}", options.version.node_version);
    Ok(())
}
