//! The payments of the last blocks: each block's operations by type.
//!
//! ```bash
//! ROSETTA_URL=http://127.0.0.1:3087 BLOCKS=10 cargo run --example scan_blocks
//! ```

use mina_rosetta_sdk::{OperationType, PartialBlockIdentifier, RosettaClient};

#[tokio::main]
async fn main() -> mina_rosetta_sdk::Result<()> {
    let url = std::env::var("ROSETTA_URL").unwrap_or_else(|_| "http://127.0.0.1:3087".into());
    let count: i64 = std::env::var("BLOCKS")
        .ok()
        .and_then(|n| n.parse().ok())
        .unwrap_or(10);
    let client = RosettaClient::discover(&url).await?;

    let status = client.network_status().await?;
    let tip = status.current_block_identifier.index;
    let first = (tip - count + 1).max(status.genesis_block_identifier.index);
    for index in first..=tip {
        let Some(block) = client
            .block(PartialBlockIdentifier::at_index(index))
            .await?
            .block
        else {
            continue;
        };
        let creator = block.metadata.map(|m| m.creator).unwrap_or_default();
        println!("block {index} by {creator}");
        for transaction in &block.transactions {
            for operation in &transaction.operations {
                if operation.kind() == Some(OperationType::PaymentReceiverInc) {
                    let to = operation.account.as_ref().map(|a| a.address.as_str());
                    let amount = operation.amount.as_ref().map(|a| a.value.as_str());
                    println!(
                        "  {} pays {} nanomina to {}",
                        transaction.transaction_identifier.hash,
                        amount.unwrap_or("?"),
                        to.unwrap_or("?")
                    );
                }
            }
        }
    }
    Ok(())
}
