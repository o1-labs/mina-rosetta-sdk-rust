//! The client against a mock server: request bodies, retries and errors.

use mina_rosetta_sdk::*;
use serde_json::{json, Value};
use std::time::Duration;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn config(server: &MockServer) -> ClientConfig {
    ClientConfig {
        base_url: format!("{}/", server.uri()),
        network: NetworkIdentifier::mina("testnet"),
        retries: 3,
        retry_delay: Duration::from_millis(5),
        timeout: Duration::from_secs(5),
        ..Default::default()
    }
}

fn client(server: &MockServer) -> RosettaClient {
    RosettaClient::with_config(config(server)).unwrap()
}

fn network() -> Value {
    json!({ "blockchain": "mina", "network": "testnet" })
}

fn status_body() -> Value {
    json!({
        "current_block_identifier": { "index": 7, "hash": "3NTIP" },
        "current_block_timestamp": 1_700_000_000_000_i64,
        "genesis_block_identifier": { "index": 1, "hash": "3NGENESIS" },
        "peers": []
    })
}

fn api_error(code: u32, retriable: bool) -> Value {
    json!({
        "code": code,
        "message": "Chain info missing",
        "retriable": retriable,
        "details": { "body": "the daemon is bootstrapping" }
    })
}

#[test]
fn the_default_config_is_mainnet_on_the_local_port() {
    let config = ClientConfig::default();
    assert_eq!(config.base_url, "http://127.0.0.1:3087");
    assert_eq!(config.network, NetworkIdentifier::mina("mainnet"));
    assert_eq!(config.retries, 3);
    assert_eq!(config.retry_delay, Duration::from_secs(5));
    assert_eq!(config.timeout, Duration::from_secs(30));
}

#[test]
fn a_config_without_attempts_or_time_is_refused() {
    let no_retries = ClientConfig {
        retries: 0,
        ..Default::default()
    };
    assert!(matches!(
        RosettaClient::with_config(no_retries),
        Err(Error::Config(_))
    ));
    let no_timeout = ClientConfig {
        timeout: Duration::ZERO,
        ..Default::default()
    };
    assert!(matches!(
        RosettaClient::with_config(no_timeout),
        Err(Error::Config(_))
    ));
}

#[tokio::test]
async fn every_request_carries_the_network() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/network/status"))
        .and(body_json(json!({ "network_identifier": network() })))
        .respond_with(ResponseTemplate::new(200).set_body_json(status_body()))
        .expect(1)
        .mount(&server)
        .await;

    let status = client(&server).network_status().await.unwrap();
    assert_eq!(status.current_block_identifier.index, 7);
    assert_eq!(status.genesis_block_identifier.hash, "3NGENESIS");
}

#[tokio::test]
async fn discover_takes_the_first_network_of_the_list() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/network/list"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "network_identifiers": [{ "blockchain": "mina", "network": "rsdk" }]
        })))
        .mount(&server)
        .await;

    let client = RosettaClient::discover_with(config(&server)).await.unwrap();
    assert_eq!(client.network(), &NetworkIdentifier::mina("rsdk"));
    assert_eq!(client.base_url(), server.uri());
}

#[tokio::test]
async fn discover_fails_when_the_list_is_empty() {
    let server = MockServer::start().await;
    Mock::given(path("/network/list"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "network_identifiers": [] })),
        )
        .mount(&server)
        .await;

    let error = RosettaClient::discover_with(config(&server))
        .await
        .unwrap_err();
    assert!(matches!(error, Error::NoNetwork(_)));
}

#[tokio::test]
async fn a_balance_request_names_the_account_and_the_block() {
    let server = MockServer::start().await;
    Mock::given(path("/account/balance"))
        .and(body_json(json!({
            "network_identifier": network(),
            "account_identifier": {
                "address": "B62qa",
                "metadata": { "token_id": DEFAULT_TOKEN_ID }
            },
            "block_identifier": { "index": 5, "hash": "3NFIVE" }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "block_identifier": { "index": 5, "hash": "3NFIVE" },
            "balances": [{
                "value": "1000",
                "currency": { "symbol": "MINA", "decimals": 9 },
                "metadata": { "locked_balance": 400, "liquid_balance": 600, "total_balance": 1000 }
            }],
            "metadata": { "created_via_historical_lookup": true, "nonce": "3" }
        })))
        .mount(&server)
        .await;

    let block = BlockIdentifier {
        index: 5,
        hash: "3NFIVE".to_string(),
    };
    let account = AccountIdentifier::with_token("B62qa", DEFAULT_TOKEN_ID);
    let response = client(&server)
        .account_balance(account, Some(block.into()))
        .await
        .unwrap();
    let balance = response.balance().unwrap();
    assert_eq!(balance.total().unwrap(), 1000);
    assert_eq!(
        balance.metadata,
        Some(BalanceMetadata {
            locked_balance: 400,
            liquid_balance: 600,
            total_balance: 1000,
        })
    );
    assert_eq!(response.nonce(), Some(3));
}

#[tokio::test]
async fn a_search_sends_only_the_conditions_it_has() {
    let server = MockServer::start().await;
    Mock::given(path("/search/transactions"))
        .and(body_json(json!({
            "network_identifier": network(),
            "address": "B62qa",
            "type": "payment_source_dec",
            "limit": 2
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "transactions": [],
            "total_count": 0
        })))
        .mount(&server)
        .await;

    let search = TransactionSearch::default()
        .address("B62qa")
        .operation_type(OperationType::PaymentSourceDec.as_str())
        .limit(2);
    let found = client(&server).search_transactions(search).await.unwrap();
    assert_eq!(found.total_count, 0);
    assert_eq!(found.next_offset, None);
}

#[tokio::test]
async fn a_rosetta_error_that_is_not_retriable_is_returned_at_once() {
    let server = MockServer::start().await;
    Mock::given(path("/network/status"))
        .respond_with(ResponseTemplate::new(500).set_body_json(api_error(3, false)))
        .expect(1)
        .mount(&server)
        .await;

    let error = client(&server).network_status().await.unwrap_err();
    let api = error.api_error().expect("a Rosetta error");
    assert_eq!(api.code, 3);
    assert!(!api.retriable);
    assert!(matches!(error, Error::Api { status: 500, .. }));
    assert_eq!(
        error.to_string(),
        r#"/network/status: [3] Chain info missing {"body":"the daemon is bootstrapping"}"#
    );
}

#[tokio::test]
async fn a_retriable_rosetta_error_is_retried() {
    let server = MockServer::start().await;
    Mock::given(path("/network/status"))
        .respond_with(ResponseTemplate::new(500).set_body_json(api_error(3, true)))
        .up_to_n_times(2)
        .expect(2)
        .mount(&server)
        .await;
    Mock::given(path("/network/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(status_body()))
        .expect(1)
        .mount(&server)
        .await;

    let status = client(&server).network_status().await.unwrap();
    assert_eq!(status.current_block_identifier.hash, "3NTIP");
}

#[tokio::test]
async fn a_retriable_rosetta_error_is_returned_after_the_last_attempt() {
    let server = MockServer::start().await;
    Mock::given(path("/network/status"))
        .respond_with(ResponseTemplate::new(500).set_body_json(api_error(3, true)))
        .expect(3)
        .mount(&server)
        .await;

    let error = client(&server).network_status().await.unwrap_err();
    assert!(error.api_error().is_some_and(|api| api.retriable));
}

#[tokio::test]
async fn a_server_error_without_a_rosetta_body_is_retried() {
    let server = MockServer::start().await;
    Mock::given(path("/mempool"))
        .respond_with(ResponseTemplate::new(502).set_body_string("Bad Gateway"))
        .expect(3)
        .mount(&server)
        .await;

    let error = client(&server).mempool().await.unwrap_err();
    assert!(
        matches!(&error, Error::Http { status: 502, body, .. } if body == "Bad Gateway"),
        "{error:?}"
    );
}

#[tokio::test]
async fn a_client_error_without_a_rosetta_body_is_not_retried() {
    let server = MockServer::start().await;
    Mock::given(path("/mempool"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .expect(1)
        .mount(&server)
        .await;

    let error = client(&server).mempool().await.unwrap_err();
    assert!(
        matches!(error, Error::Http { status: 404, .. }),
        "{error:?}"
    );
}

#[tokio::test]
async fn an_unexpected_answer_is_a_decode_error() {
    let server = MockServer::start().await;
    Mock::given(path("/network/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "tip": 7 })))
        .expect(1)
        .mount(&server)
        .await;

    let error = client(&server).network_status().await.unwrap_err();
    assert!(
        matches!(&error, Error::Decode { endpoint, .. } if endpoint == "/network/status"),
        "{error:?}"
    );
}

#[tokio::test]
async fn an_unreachable_server_is_tried_each_time() {
    // a port nothing listens on: bind one, then free it
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let client = RosettaClient::with_config(ClientConfig {
        base_url: format!("http://127.0.0.1:{port}"),
        retries: 2,
        retry_delay: Duration::from_millis(5),
        ..Default::default()
    })
    .unwrap();

    let error = client.network_list().await.unwrap_err();
    assert!(
        matches!(error, Error::Connection { attempts: 2, .. }),
        "{error:?}"
    );
}

#[tokio::test]
async fn headers_of_the_config_go_with_every_request() {
    let server = MockServer::start().await;
    Mock::given(path("/network/list"))
        .and(wiremock::matchers::header("x-api-key", "secret"))
        .and(wiremock::matchers::header(
            "content-type",
            "application/json",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "network_identifiers": [] })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let mut config = config(&server);
    config
        .headers
        .insert("x-api-key", header::HeaderValue::from_static("secret"));
    let list = RosettaClient::with_config(config)
        .unwrap()
        .network_list()
        .await
        .unwrap();
    assert!(list.network_identifiers.is_empty());
}

#[tokio::test]
async fn the_construction_flow_sends_what_the_server_returned() {
    let server = MockServer::start().await;
    let operations = Transfer::new("B62qa", "B62qb", 5, 1).operations();
    let options = json!({ "sender": "B62qa", "receiver": "B62qb", "token_id": DEFAULT_TOKEN_ID });
    let metadata = json!({ "sender": "B62qa", "nonce": "4" });
    let key = PublicKey::pallas("ABCD");

    Mock::given(path("/construction/preprocess"))
        .and(body_json(json!({
            "network_identifier": network(),
            "operations": operations,
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "options": options })))
        .mount(&server)
        .await;
    Mock::given(path("/construction/metadata"))
        .and(body_json(json!({
            "network_identifier": network(),
            "options": options,
            "public_keys": [{ "hex_bytes": "ABCD", "curve_type": "pallas" }],
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "metadata": metadata,
            "suggested_fee": [{ "value": "100", "currency": { "symbol": "MINA", "decimals": 9 } }]
        })))
        .mount(&server)
        .await;
    Mock::given(path("/construction/payloads"))
        .and(body_json(json!({
            "network_identifier": network(),
            "operations": operations,
            "metadata": metadata,
            "public_keys": [{ "hex_bytes": "ABCD", "curve_type": "pallas" }],
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "unsigned_transaction": "{}",
            "payloads": [{
                "account_identifier": { "address": "B62qa", "metadata": { "token_id": DEFAULT_TOKEN_ID } },
                "hex_bytes": "00",
                "signature_type": "schnorr_poseidon"
            }]
        })))
        .mount(&server)
        .await;

    let client = client(&server);
    let preprocess = client
        .construction_preprocess(operations.clone(), None)
        .await
        .unwrap();
    let suggested = client
        .construction_metadata(preprocess.options, vec![key.clone()])
        .await
        .unwrap();
    assert_eq!(suggested.suggested_fee[0].signed_value().unwrap(), 100);
    let payloads = client
        .construction_payloads(operations, Some(suggested.metadata), vec![key])
        .await
        .unwrap();
    assert_eq!(
        payloads.payloads[0].signature_type,
        Some(SignatureType::SchnorrPoseidon)
    );
}

#[tokio::test]
async fn post_reaches_endpoints_the_client_does_not_wrap() {
    let server = MockServer::start().await;
    Mock::given(path("/call"))
        .and(body_json(json!({ "method": "x" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "result": 1 })))
        .mount(&server)
        .await;

    let answer: Value = client(&server)
        .post("/call", &json!({ "method": "x" }))
        .await
        .unwrap();
    assert_eq!(answer["result"], 1);
}
