//! Rust SDK for the Rosetta API of [Mina Protocol](https://minaprotocol.com):
//! `mina-rosetta`, which implements the
//! [Coinbase Mesh](https://docs.cdp.coinbase.com/mesh/docs/welcome)
//! (formerly Rosetta) specification on top of a daemon and an archive.
//!
//! Companion to [`mina-sdk`](https://crates.io/crates/mina-sdk) (the
//! daemon's GraphQL API) and
//! [`mina-archive-sdk`](https://crates.io/crates/mina-archive-sdk) (the
//! Archive-Node-API), and the Rust sibling of
//! [`@o1-labs/mina-rosetta-sdk`](https://github.com/o1-labs/mina-rosetta-sdk-js).
//!
//! # Quick start
//!
//! ```no_run
//! # async fn example() -> mina_rosetta_sdk::Result<()> {
//! use mina_rosetta_sdk::{PartialBlockIdentifier, RosettaClient};
//!
//! // The network the server serves, from `/network/list`
//! let client = RosettaClient::discover("http://127.0.0.1:3087").await?;
//!
//! let status = client.network_status().await?;
//! let tip = status.current_block_identifier;
//! println!("tip {} {}", tip.index, tip.hash);
//!
//! let response = client.account_balance("B62q...", Some(tip.into())).await?;
//! if let Some(balance) = response.balance() {
//!     println!("{} nanomina, nonce {:?}", balance.value, response.nonce());
//!     if let Some(split) = &balance.metadata {
//!         println!("{} locked, {} liquid", split.locked_balance, split.liquid_balance);
//!     }
//! }
//!
//! let block = client.block(PartialBlockIdentifier::at_index(1)).await?;
//! # let _ = block;
//! # Ok(())
//! # }
//! ```
//!
//! # Mina's metadata
//!
//! The Rosetta types leave metadata free-form; this crate types what Mina
//! puts there: the locked, liquid and total balance ([`BalanceMetadata`]),
//! the nonce ([`AccountBalanceMetadata`]), token ids ([`TokenMetadata`]) and
//! block creators ([`BlockMetadata`]). Operation types are
//! [`OperationType`].
//!
//! # Construction
//!
//! [`Transfer`] and [`Delegation`] make the operations of a payment and a
//! delegation. The flow is preprocess → metadata → payloads → sign →
//! combine → submit; signing is not part of this crate (use `mina-signer`
//! or `mina-ocaml-signer`).
//!
//! ```no_run
//! # async fn example(client: mina_rosetta_sdk::RosettaClient) -> mina_rosetta_sdk::Result<()> {
//! use mina_rosetta_sdk::{PublicKey, Transfer};
//!
//! let operations = Transfer::new("B62qsender", "B62qreceiver", 1_000_000_000, 10_000_000)
//!     .operations();
//! let public_keys = vec![PublicKey::pallas("<sender's key as hex>")];
//! let options = client.construction_preprocess(operations.clone(), None).await?.options;
//! let metadata = client.construction_metadata(options, public_keys.clone()).await?;
//! let payloads = client
//!     .construction_payloads(operations, Some(metadata.metadata), public_keys)
//!     .await?;
//! // sign payloads.payloads, then construction_combine and construction_submit
//! # let _ = payloads;
//! # Ok(())
//! # }
//! ```
//!
//! # Errors
//!
//! A Rosetta error from the server is [`Error::Api`], with the server's
//! [`ApiError`]: match on its `code`. A request is retried when the server
//! cannot be reached, answers 5xx without a Rosetta error, or marks its
//! error `retriable`; see [`ClientConfig`].
//!
//! # Runtime
//!
//! The client is async and needs a Tokio runtime with the time driver
//! (`#[tokio::main]` has it; `Builder::new_current_thread()` needs
//! `enable_time()` and `enable_io()`).

mod client;
mod client_config;
mod constants;
pub mod error;
pub mod models;
pub mod operations;
pub mod requests;

pub use client::RosettaClient;
pub use client_config::ClientConfig;
pub use constants::{
    BLOCKCHAIN, DEFAULT_PORT, DEFAULT_TOKEN_ID, DEVNET, MAINNET, MINA_DECIMALS, MINA_SYMBOL,
};
pub use error::{Error, Result};
pub use models::*;
pub use operations::{Delegation, Transfer};
pub use requests::TransactionSearch;
pub use reqwest::header;
