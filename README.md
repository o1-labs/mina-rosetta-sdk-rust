# Mina Rosetta Rust SDK

Rust SDK for the Rosetta API of [Mina Protocol](https://minaprotocol.com):
`mina-rosetta`, Mina's implementation of the
[Coinbase Mesh](https://docs.cdp.coinbase.com/mesh/docs/welcome) (formerly
Rosetta) specification.

It is the Rust sibling of
[`@o1-labs/mina-rosetta-sdk`](https://github.com/o1-labs/mina-rosetta-sdk-js),
and a companion to [`mina-sdk`](https://github.com/o1-labs/mina-sdk-rust)
(the daemon's GraphQL API) and
[`mina-archive-sdk`](https://github.com/o1-labs/mina-archive-sdk-rust)
(the Archive-Node-API).

## Features

- **Async client** for the Data and Construction APIs, one method per endpoint
- **Typed Mina metadata**: locked, liquid and total balances, nonces, token
  ids, block creators and operation types, which the generic Mesh types leave
  as free-form JSON
- **Operation builders** for payments and delegations
- **Retries** on unreachable servers, 5xx answers and errors the server marks
  `retriable`
- **No OpenSSL** with the `rustls-tls` feature

## Installation

```toml
[dependencies]
mina-rosetta-sdk = "0.1"
```

`https://` endpoints use the system's OpenSSL (feature `native-tls`, on by
default). To build without OpenSSL:

```toml
[dependencies]
mina-rosetta-sdk = { version = "0.1", default-features = false, features = ["rustls-tls"] }
```

## Quick start

```rust
use mina_rosetta_sdk::{PartialBlockIdentifier, RosettaClient};

#[tokio::main]
async fn main() -> mina_rosetta_sdk::Result<()> {
    // the network the server serves, from /network/list
    let client = RosettaClient::discover("http://127.0.0.1:3087").await?;

    let status = client.network_status().await?;
    println!("tip {}", status.current_block_identifier.index);

    let response = client.account_balance("B62q...", None).await?;
    let balance = response.balance().expect("one balance");
    println!("{} nanomina, nonce {:?}", balance.value, response.nonce());
    if let Some(split) = &balance.metadata {
        println!("locked {}, liquid {}", split.locked_balance, split.liquid_balance);
    }

    let block = client.block(PartialBlockIdentifier::at_index(2)).await?;
    for transaction in block.block.unwrap().transactions {
        for operation in transaction.operations {
            println!("{:?} {:?}", operation.kind(), operation.amount);
        }
    }
    Ok(())
}
```

`RosettaClient::new(url, "devnet")` names the network instead of asking the
server.

## API

### Data API

| Method | Endpoint |
| --- | --- |
| `network_list()` | `/network/list` |
| `network_status()` | `/network/status` |
| `network_options()` | `/network/options` |
| `block(block)` | `/block` |
| `account_balance(account, block)` | `/account/balance` |
| `mempool()` | `/mempool` |
| `mempool_transaction(hash)` | `/mempool/transaction` |
| `search_transactions(search)` | `/search/transactions` |

### Construction API

| Method | Endpoint |
| --- | --- |
| `construction_derive(public_key)` | `/construction/derive` |
| `construction_preprocess(operations, metadata)` | `/construction/preprocess` |
| `construction_metadata(options, public_keys)` | `/construction/metadata` |
| `construction_payloads(operations, metadata, public_keys)` | `/construction/payloads` |
| `construction_combine(unsigned_transaction, signatures)` | `/construction/combine` |
| `construction_parse(signed, transaction)` | `/construction/parse` |
| `construction_hash(signed_transaction)` | `/construction/hash` |
| `construction_submit(signed_transaction)` | `/construction/submit` |

`post(endpoint, body)` sends any other request with the same retries.

## Sending a payment

`Transfer` makes the three operations Mina's server expects (`fee_payment`,
`payment_source_dec`, `payment_receiver_inc`); `Delegation` makes the two of a
delegation. This crate does not sign: use
[`mina-signer`](https://www.npmjs.com/package/mina-signer) or
`mina-ocaml-signer`, which the `mina-rosetta` package installs.

```rust
use mina_rosetta_sdk::{PublicKey, RosettaClient, Signature, SignatureType, Transfer};

async fn pay(client: &RosettaClient, key_hex: &str) -> mina_rosetta_sdk::Result<String> {
    let public_key = PublicKey::pallas(key_hex);
    let operations = Transfer::new("B62qsender", "B62qreceiver", 1_000_000_000, 10_000_000)
        .operations();

    let options = client.construction_preprocess(operations.clone(), None).await?.options;
    let metadata = client.construction_metadata(options, vec![public_key.clone()]).await?;
    let payloads = client
        .construction_payloads(operations, Some(metadata.metadata), vec![public_key.clone()])
        .await?;

    let signature = Signature {
        signing_payload: payloads.payloads[0].clone(),
        public_key,
        signature_type: SignatureType::SchnorrPoseidon,
        hex_bytes: sign(&payloads.unsigned_transaction), // your signer
    };
    let signed = client
        .construction_combine(&payloads.unsigned_transaction, vec![signature])
        .await?
        .signed_transaction;
    let submitted = client.construction_submit(&signed).await?;
    Ok(submitted.transaction_identifier.hash)
}
```

## Configuration

```rust
use mina_rosetta_sdk::{header::HeaderValue, ClientConfig, NetworkIdentifier, RosettaClient};
use std::time::Duration;

let mut config = ClientConfig {
    base_url: "https://rosetta.example".to_string(),
    network: NetworkIdentifier::mina("devnet"),
    retries: 5,                          // attempts in all, default 3
    retry_delay: Duration::from_secs(10), // default 5 s
    timeout: Duration::from_secs(60),     // per attempt, default 30 s
    ..Default::default()
};
config.headers.insert("x-api-key", HeaderValue::from_static("..."));
let client = RosettaClient::with_config(config)?;
```

## Errors

```rust
use mina_rosetta_sdk::Error;

match client.account_balance("B62q...", None).await {
    Ok(response) => println!("{:?}", response.balance()),
    Err(Error::Api { error, .. }) => eprintln!("rosetta error {}: {}", error.code, error.message),
    Err(Error::Connection { attempts, .. }) => eprintln!("no answer after {attempts} attempts"),
    Err(error) => eprintln!("{error}"),
}
```

A request is retried when the server cannot be reached, answers 5xx without a
Rosetta error, or returns a Rosetta error marked `retriable`. Other errors
return at once.

## Runtime

The client is async and needs a Tokio runtime with the time driver.
`#[tokio::main]` has it; `Builder::new_current_thread()` needs
`enable_time()` and `enable_io()`.

## Development

```bash
cargo test
```

The unit tests use a mock server and responses captured from a real
`mina-rosetta` (`tests/fixtures/`). The integration tests run against a live
server and skip themselves without one:

```bash
MINA_ROSETTA_URL=http://127.0.0.1:3087 cargo test --test integration_tests -- --test-threads=1
```

`scripts/capture_fixtures.py` regenerates the fixtures from a live server.
`tests/integration_tests.rs` lists the variables for the construction test,
which signs with `mina-ocaml-signer` and submits a payment.
[minimina](https://github.com/o1-labs/minimina) starts a local network with
a rosetta node (`role = "rosetta"`).

## License

Apache-2.0
