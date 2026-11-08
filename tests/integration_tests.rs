//! The client against a live Mina rosetta server.
//!
//! Each test returns at once unless `MINA_ROSETTA_URL` is set, such as
//! `http://127.0.0.1:3087`. The construction test also needs:
//!
//! - `MINA_TEST_SIGNER`: the path of `mina-ocaml-signer` (it reads
//!   `MINA_PROFILE`, so set that too);
//! - `MINA_TEST_PRIVATE_KEY`: the sender's private key as hex, as
//!   `mina-ocaml-signer hex-of-private-key-file` prints it;
//! - `MINA_TEST_RECEIVER`: the receiver's public key.
//!
//! The construction test submits a payment of 0.001 MINA.
//!
//! ```bash
//! MINA_ROSETTA_URL=http://127.0.0.1:3087 cargo test --test integration_tests -- --test-threads=1
//! ```

use mina_rosetta_sdk::*;
use std::collections::BTreeSet;
use std::env;
use std::process::Command;

fn url() -> Option<String> {
    let url = env::var("MINA_ROSETTA_URL").ok();
    if url.is_none() {
        eprintln!("MINA_ROSETTA_URL is not set: skipped");
    }
    url
}

async fn client() -> Option<RosettaClient> {
    Some(RosettaClient::discover(url()?).await.expect("discover"))
}

#[tokio::test]
async fn integration_status_and_options() {
    let Some(client) = client().await else { return };
    assert_eq!(client.network().blockchain, BLOCKCHAIN);

    let status = client.network_status().await.unwrap();
    assert!(status.current_block_identifier.index >= status.genesis_block_identifier.index);
    assert!(status.current_block_timestamp > 0);

    let options = client.network_options().await.unwrap();
    assert!(!options.version.node_version.is_empty());
    // every operation type the server knows is one this crate knows
    let unknown: Vec<_> = options
        .allow
        .operation_types
        .iter()
        .filter(|name| name.parse::<OperationType>().is_err())
        .collect();
    assert!(unknown.is_empty(), "unknown operation types {unknown:?}");
    let statuses: BTreeSet<_> = options
        .allow
        .operation_statuses
        .iter()
        .map(|s| s.status.as_str())
        .collect();
    assert!(statuses.contains("Success"), "{statuses:?}");
}

#[tokio::test]
async fn integration_every_block_decodes() {
    let Some(client) = client().await else { return };
    let status = client.network_status().await.unwrap();
    let genesis = status.genesis_block_identifier.index;
    let tip = status.current_block_identifier;
    let mut parent = None;
    for index in genesis..=tip.index {
        let block = client
            .block(PartialBlockIdentifier::at_index(index))
            .await
            .unwrap()
            .block
            .expect("a block at every height up to the tip");
        assert_eq!(block.block_identifier.index, index);
        if let Some(parent) = parent {
            assert_eq!(block.parent_block_identifier, parent);
        }
        assert!(block.metadata.is_some_and(|m| m.creator.starts_with("B62")));
        for transaction in &block.transactions {
            for operation in &transaction.operations {
                assert!(
                    operation.kind().is_some(),
                    "unknown type {}",
                    operation.operation_type
                );
            }
        }
        parent = Some(block.block_identifier);
    }
}

#[tokio::test]
async fn integration_balances_split_into_locked_and_liquid() {
    let Some(client) = client().await else { return };
    let status = client.network_status().await.unwrap();
    let genesis = client
        .block(status.genesis_block_identifier.clone().into())
        .await
        .unwrap()
        .block
        .unwrap();
    let creator = genesis.metadata.unwrap().creator;

    for block in [None, Some(status.current_block_identifier.into())] {
        let response = client
            .account_balance(creator.as_str(), block)
            .await
            .unwrap();
        let balance = response.balance().expect("one balance");
        assert_eq!(balance.currency, Currency::mina());
        let split = balance.metadata.as_ref().expect("Mina's split");
        assert_eq!(
            split.locked_balance + split.liquid_balance,
            split.total_balance
        );
        assert_eq!(balance.total().unwrap(), split.total_balance);
        assert!(response.nonce().is_some());
    }
}

#[tokio::test]
async fn integration_an_unknown_account_is_a_rosetta_error() {
    let Some(client) = client().await else { return };
    let error = client.account_balance("B62qnot", None).await.unwrap_err();
    let api = error.api_error().expect("a Rosetta error");
    assert!(!api.message.is_empty());
}

#[tokio::test]
async fn integration_search_pages() {
    let Some(client) = client().await else { return };
    let page = client
        .search_transactions(TransactionSearch::default().limit(1))
        .await
        .unwrap();
    assert!(page.transactions.len() <= 1);
    if page.total_count > 1 {
        assert_eq!(page.next_offset, Some(1));
    }
}

/// Runs `mina-ocaml-signer` with `args` and returns the last word it prints.
fn signer(args: &[&str]) -> String {
    let program = env::var("MINA_TEST_SIGNER").unwrap();
    let out = Command::new(&program).args(args).output().expect("signer");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8(out.stdout).unwrap();
    text.split_whitespace().last().unwrap().to_string()
}

/// The public key of a private key, as hex: `derive-public-key` prints the
/// word after "Public Key:".
fn public_key_hex(private_key: &str) -> String {
    let program = env::var("MINA_TEST_SIGNER").unwrap();
    let out = Command::new(&program)
        .args(["derive-public-key", "--private-key", private_key])
        .output()
        .expect("signer");
    let text = String::from_utf8(out.stdout).unwrap();
    let words: Vec<_> = text.split_whitespace().collect();
    let at = words
        .windows(2)
        .position(|w| w == ["Public", "Key:"])
        .expect("Public Key:");
    words[at + 2].to_string()
}

#[tokio::test]
async fn integration_construction_sends_a_payment() {
    let Some(client) = client().await else { return };
    let (Ok(private_key), Ok(receiver), Ok(_)) = (
        env::var("MINA_TEST_PRIVATE_KEY"),
        env::var("MINA_TEST_RECEIVER"),
        env::var("MINA_TEST_SIGNER"),
    ) else {
        eprintln!(
            "MINA_TEST_SIGNER, MINA_TEST_PRIVATE_KEY or MINA_TEST_RECEIVER is not set: skipped"
        );
        return;
    };
    let public_key = PublicKey::pallas(public_key_hex(&private_key));
    let sender = client
        .construction_derive(public_key.clone())
        .await
        .unwrap()
        .account_identifier
        .expect("an account")
        .address;

    let fee = 10_000_000;
    let operations = Transfer::new(&sender, &receiver, 1_000_000, fee).operations();
    let mut memo = Metadata::new();
    memo.insert("memo".into(), "mina-rosetta-sdk".into());
    let options = client
        .construction_preprocess(operations.clone(), Some(memo))
        .await
        .unwrap()
        .options;
    let metadata = client
        .construction_metadata(options, vec![public_key.clone()])
        .await
        .unwrap();
    let payloads = client
        .construction_payloads(
            operations.clone(),
            Some(metadata.metadata),
            vec![public_key.clone()],
        )
        .await
        .unwrap();

    // the unsigned transaction has the operations asked for
    let parsed = client
        .construction_parse(false, &payloads.unsigned_transaction)
        .await
        .unwrap();
    assert_eq!(parsed.operations, operations);

    let signature = Signature {
        signing_payload: payloads.payloads[0].clone(),
        public_key,
        signature_type: SignatureType::SchnorrPoseidon,
        hex_bytes: signer(&[
            "sign",
            "--private-key",
            &private_key,
            "--unsigned-transaction",
            &payloads.unsigned_transaction,
        ]),
    };
    let signed = client
        .construction_combine(&payloads.unsigned_transaction, vec![signature])
        .await
        .unwrap()
        .signed_transaction;
    let parsed = client.construction_parse(true, &signed).await.unwrap();
    assert_eq!(parsed.operations, operations);
    assert_eq!(parsed.account_identifier_signers[0].address, sender);

    let hash = client.construction_hash(&signed).await.unwrap();
    let submitted = client.construction_submit(&signed).await.unwrap();
    assert_eq!(
        submitted.transaction_identifier,
        hash.transaction_identifier
    );

    let pooled = client
        .mempool_transaction(&hash.transaction_identifier.hash)
        .await
        .unwrap();
    assert_eq!(
        pooled.transaction.transaction_identifier,
        hash.transaction_identifier
    );
}
