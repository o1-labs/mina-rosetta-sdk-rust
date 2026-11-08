//! The balance of an account, split into locked and liquid, and its nonce.
//!
//! ```bash
//! ROSETTA_URL=http://127.0.0.1:3087 ADDRESS=B62q... cargo run --example account_balance
//! ```

use mina_rosetta_sdk::RosettaClient;

#[tokio::main]
async fn main() -> mina_rosetta_sdk::Result<()> {
    let url = std::env::var("ROSETTA_URL").unwrap_or_else(|_| "http://127.0.0.1:3087".into());
    let address = std::env::var("ADDRESS").expect("ADDRESS: a public key");
    let client = RosettaClient::discover(&url).await?;

    let response = client.account_balance(address.as_str(), None).await?;
    let block = &response.block_identifier;
    println!("at block {} {}", block.index, block.hash);
    if let Some(balance) = response.balance() {
        println!("total   {} nanomina", balance.value);
        if let Some(split) = &balance.metadata {
            println!("locked  {}", split.locked_balance);
            println!("liquid  {}", split.liquid_balance);
        }
    }
    println!("nonce   {:?}", response.nonce());
    Ok(())
}
