//! Responses of a real `mina-rosetta` (4.0.4, captured on a local network
//! made by minimina) decode, and keep every field: written back, each one is
//! the server's JSON, up to empty lists and nulls.

use mina_rosetta_sdk::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

fn fixture(name: &str) -> Value {
    let path = format!("{}/tests/fixtures/{name}.json", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    serde_json::from_str(&text).unwrap()
}

/// `value` without nulls, empty lists and the fields they leave empty.
fn normalized(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| (key, normalized(value)))
                .filter(|(_, value)| !is_empty(value))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(normalized).collect()),
        other => other,
    }
}

fn is_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Array(items) => items.is_empty(),
        _ => false,
    }
}

/// Decodes the fixture `name` as `T` and checks that `T` keeps all of it.
fn decode<T: DeserializeOwned + Serialize>(name: &str) -> T {
    let json = fixture(name);
    let decoded: T = serde_json::from_value(json.clone())
        .unwrap_or_else(|e| panic!("{name} does not decode: {e}"));
    let written = serde_json::to_value(&decoded).unwrap();
    assert_eq!(
        normalized(written),
        normalized(json),
        "{name} loses or changes fields"
    );
    decoded
}

#[test]
fn network_list() {
    let list: NetworkListResponse = decode("network_list");
    assert_eq!(list.network_identifiers[0].blockchain, BLOCKCHAIN);
}

#[test]
fn network_status() {
    let status: NetworkStatusResponse = decode("network_status");
    assert!(status.current_block_identifier.index > status.genesis_block_identifier.index);
    let sync = status.sync_status.unwrap();
    assert_eq!(sync.stage.as_deref(), Some("Synced"));
    assert_eq!(sync.synced, Some(true));
    assert!(!status.peers.is_empty());
}

#[test]
fn network_options_list_only_known_operation_types() {
    let options: NetworkOptionsResponse = decode("network_options");
    for name in &options.allow.operation_types {
        assert!(name.parse::<OperationType>().is_ok(), "{name}");
    }
    assert!(options.allow.historical_balance_lookup);
    assert!(options.allow.errors.iter().any(|e| e.code > 0));
}

#[test]
fn a_block_with_a_payment() {
    let response: BlockResponse = decode("block_with_payment");
    let block = response.block.unwrap();
    assert!(block.metadata.unwrap().creator.starts_with("B62"));
    let operations: Vec<_> = block
        .transactions
        .iter()
        .flat_map(|t| &t.operations)
        .collect();
    assert!(operations.iter().all(|o| o.kind().is_some()));
    let paid = operations
        .iter()
        .find(|o| o.kind() == Some(OperationType::PaymentReceiverInc))
        .unwrap();
    assert!(paid.amount.as_ref().unwrap().signed_value().unwrap() > 0);
    assert_eq!(paid.status.as_deref(), Some("Success"));
}

#[test]
fn the_genesis_block() {
    let response: BlockResponse = decode("block_genesis");
    let block = response.block.unwrap();
    assert_eq!(block.block_identifier.index, 1);
}

#[test]
fn account_balance() {
    let response: AccountBalanceResponse = decode("account_balance");
    let balance = response.balance().unwrap();
    let split = balance.metadata.as_ref().unwrap();
    assert_eq!(
        split.locked_balance + split.liquid_balance,
        split.total_balance
    );
    assert_eq!(balance.total().unwrap(), split.total_balance);
    assert_eq!(balance.currency, Currency::mina());
    let metadata = response.metadata.unwrap();
    assert!(!metadata.created_via_historical_lookup);
}

#[test]
fn a_historical_account_balance() {
    let response: AccountBalanceResponse = decode("account_balance_historical");
    assert_eq!(response.block_identifier.index, 1);
    assert_eq!(response.nonce(), Some(0));
}

#[test]
fn a_rosetta_error() {
    let error: ApiError = decode("error_bad_address");
    assert!(error.code > 0);
    assert!(!error.message.is_empty());
}

#[test]
fn construction_derive() {
    let derived: ConstructionDeriveResponse = decode("construction_derive");
    let account = derived.account_identifier.unwrap();
    assert!(account.address.starts_with("B62"));
}

#[test]
fn construction_preprocess_and_metadata() {
    let preprocess: ConstructionPreprocessResponse = decode("construction_preprocess");
    let options = preprocess.options.unwrap();
    assert_eq!(options["memo"], "rosetta sdk");
    let metadata: ConstructionMetadataResponse = decode("construction_metadata");
    assert!(metadata.metadata.contains_key("nonce"));
    assert!(metadata.suggested_fee[0].signed_value().unwrap() > 0);
}

#[test]
fn construction_payloads() {
    let payloads: ConstructionPayloadsResponse = decode("construction_payloads");
    assert_eq!(payloads.payloads.len(), 1);
    assert_eq!(
        payloads.payloads[0].signature_type,
        Some(SignatureType::SchnorrPoseidon)
    );
}

#[test]
fn construction_parse_gives_back_the_transfer() {
    let unsigned: ConstructionParseResponse = decode("construction_parse_unsigned");
    let signed: ConstructionParseResponse = decode("construction_parse_signed");
    assert_eq!(unsigned.operations, signed.operations);
    assert!(unsigned.account_identifier_signers.is_empty());
    assert_eq!(signed.account_identifier_signers.len(), 1);

    // the operations Transfer makes for the same payment
    let sender = &signed.account_identifier_signers[0].address;
    let receiver = signed.operations[2].account.as_ref().unwrap();
    let transfer = Transfer::new(sender, &receiver.address, 1_000_000_000, 10_000_000);
    assert_eq!(signed.operations, transfer.operations());
}

#[test]
fn construction_combine_hash_and_submit() {
    let combined: ConstructionCombineResponse = decode("construction_combine");
    assert!(!combined.signed_transaction.is_empty());
    let hash: TransactionIdentifierResponse = decode("construction_hash");
    let submitted: TransactionIdentifierResponse = decode("construction_submit");
    assert_eq!(
        hash.transaction_identifier,
        submitted.transaction_identifier
    );
}

#[test]
fn mempool() {
    let pool: MempoolResponse = decode("mempool");
    assert!(!pool.transaction_identifiers.is_empty());
    let pooled: MempoolTransactionResponse = decode("mempool_transaction");
    assert!(!pooled.transaction.operations.is_empty());
}

#[test]
fn search_transactions() {
    let page: SearchTransactionsResponse = decode("search_transactions");
    assert!(page.transactions.len() <= 2);
    assert!(page.total_count >= page.transactions.len() as i64);
}
