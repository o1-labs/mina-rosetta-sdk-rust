use crate::error::{Error, Result};
use crate::models::{
    AccountBalanceResponse, AccountIdentifier, ApiError, BlockResponse,
    ConstructionCombineResponse, ConstructionDeriveResponse, ConstructionMetadataResponse,
    ConstructionParseResponse, ConstructionPayloadsResponse, ConstructionPreprocessResponse,
    MempoolResponse, MempoolTransactionResponse, Metadata, NetworkIdentifier, NetworkListResponse,
    NetworkOptionsResponse, NetworkStatusResponse, Operation, PartialBlockIdentifier, PublicKey,
    SearchTransactionsResponse, Signature, TransactionIdentifier, TransactionIdentifierResponse,
};
use crate::requests::{
    AccountBalanceRequest, BlockRequest, ConstructionCombineRequest, ConstructionDeriveRequest,
    ConstructionMetadataRequest, ConstructionParseRequest, ConstructionPayloadsRequest,
    ConstructionPreprocessRequest, MempoolTransactionRequest, MetadataRequest, NetworkRequest,
    SearchTransactionsRequest, SignedTransactionRequest, TransactionSearch,
};
use crate::ClientConfig;
use reqwest::header::{HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, warn};

/// The longest part of a non-Rosetta error body an [`Error::Http`] keeps.
const BODY_LIMIT: usize = 500;

/// A client of a Mina Rosetta server.
///
/// Every method is one endpoint, and every request carries the client's
/// network. Requests are retried as [`ClientConfig`] says.
///
/// ```no_run
/// # async fn example() -> mina_rosetta_sdk::Result<()> {
/// use mina_rosetta_sdk::RosettaClient;
///
/// let client = RosettaClient::new("http://127.0.0.1:3087", "devnet")?;
/// let status = client.network_status().await?;
/// println!("tip {}", status.current_block_identifier.index);
///
/// let balance = client.account_balance("B62q...", None).await?;
/// println!("nonce {:?}", balance.nonce());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RosettaClient {
    config: ClientConfig,
    http: reqwest::Client,
}

impl RosettaClient {
    /// A client of the server at `base_url` for the Mina network `network`,
    /// with the default retries and timeout.
    pub fn new(base_url: impl Into<String>, network: impl Into<String>) -> Result<Self> {
        RosettaClient::with_config(ClientConfig {
            base_url: base_url.into(),
            network: NetworkIdentifier::mina(network),
            ..Default::default()
        })
    }

    /// A client with `config`.
    pub fn with_config(mut config: ClientConfig) -> Result<Self> {
        if config.retries == 0 {
            return Err(Error::Config("retries must be at least 1"));
        }
        if config.timeout.is_zero() {
            return Err(Error::Config("timeout must be more than zero"));
        }
        config.base_url = config.base_url.trim_end_matches('/').to_string();
        let mut headers = config.headers.clone();
        let json = HeaderValue::from_static("application/json");
        headers.entry(CONTENT_TYPE).or_insert(json.clone());
        headers.entry(ACCEPT).or_insert(json);
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .build()
            .map_err(|cause| Error::Client { cause })?;
        Ok(RosettaClient { config, http })
    }

    /// A client for the network the server at `base_url` serves: the first
    /// of `/network/list`. A Mina server serves one.
    pub async fn discover(base_url: impl Into<String>) -> Result<Self> {
        RosettaClient::discover_with(ClientConfig {
            base_url: base_url.into(),
            ..Default::default()
        })
        .await
    }

    /// [`RosettaClient::discover`] with `config`, whose network it replaces.
    pub async fn discover_with(config: ClientConfig) -> Result<Self> {
        let mut client = RosettaClient::with_config(config)?;
        let network = client
            .network_list()
            .await?
            .network_identifiers
            .into_iter()
            .next()
            .ok_or_else(|| Error::NoNetwork(client.config.base_url.clone()))?;
        client.config.network = network;
        Ok(client)
    }

    /// The server's base URL, without a trailing slash.
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// The network every request is for.
    pub fn network(&self) -> &NetworkIdentifier {
        &self.config.network
    }

    /// The client's configuration.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    // Data API

    /// `/network/list`: the networks the server serves.
    pub async fn network_list(&self) -> Result<NetworkListResponse> {
        self.post("/network/list", &MetadataRequest::default())
            .await
    }

    /// `/network/status`: the tip, the genesis block and the sync state.
    pub async fn network_status(&self) -> Result<NetworkStatusResponse> {
        self.post("/network/status", &self.network_request()).await
    }

    /// `/network/options`: versions, operation types, statuses and errors.
    pub async fn network_options(&self) -> Result<NetworkOptionsResponse> {
        self.post("/network/options", &self.network_request()).await
    }

    /// `/block`: a block and its transactions. [`PartialBlockIdentifier::tip`]
    /// asks for the best tip.
    pub async fn block(&self, block: PartialBlockIdentifier) -> Result<BlockResponse> {
        let request = BlockRequest {
            network_identifier: self.network().clone(),
            block_identifier: block,
        };
        self.post("/block", &request).await
    }

    /// `/account/balance`: the balance and nonce of `account`, at the tip or
    /// at `block`. A public key is the MINA account of that key.
    pub async fn account_balance(
        &self,
        account: impl Into<AccountIdentifier>,
        block: Option<PartialBlockIdentifier>,
    ) -> Result<AccountBalanceResponse> {
        let request = AccountBalanceRequest {
            network_identifier: self.network().clone(),
            account_identifier: account.into(),
            block_identifier: block,
            currencies: Vec::new(),
        };
        self.post("/account/balance", &request).await
    }

    /// `/mempool`: the transactions in the daemon's pool.
    pub async fn mempool(&self) -> Result<MempoolResponse> {
        self.post("/mempool", &self.network_request()).await
    }

    /// `/mempool/transaction`: the pooled transaction with hash `hash`.
    pub async fn mempool_transaction(
        &self,
        hash: impl Into<String>,
    ) -> Result<MempoolTransactionResponse> {
        let request = MempoolTransactionRequest {
            network_identifier: self.network().clone(),
            transaction_identifier: TransactionIdentifier::new(hash),
        };
        self.post("/mempool/transaction", &request).await
    }

    /// `/search/transactions`: the archived transactions that match `search`.
    pub async fn search_transactions(
        &self,
        search: TransactionSearch,
    ) -> Result<SearchTransactionsResponse> {
        let request = SearchTransactionsRequest {
            network_identifier: self.network().clone(),
            search,
        };
        self.post("/search/transactions", &request).await
    }

    // Construction API: derive → preprocess → metadata → payloads → sign
    // (outside this crate) → combine → parse → hash → submit.

    /// `/construction/derive`: the account of a public key.
    pub async fn construction_derive(
        &self,
        public_key: PublicKey,
    ) -> Result<ConstructionDeriveResponse> {
        let request = ConstructionDeriveRequest {
            network_identifier: self.network().clone(),
            public_key,
            metadata: None,
        };
        self.post("/construction/derive", &request).await
    }

    /// `/construction/preprocess`: the options for
    /// [`RosettaClient::construction_metadata`]. Mina reads `memo` and
    /// `valid_until` from `metadata`.
    pub async fn construction_preprocess(
        &self,
        operations: Vec<Operation>,
        metadata: Option<Metadata>,
    ) -> Result<ConstructionPreprocessResponse> {
        let request = ConstructionPreprocessRequest {
            network_identifier: self.network().clone(),
            operations,
            metadata,
        };
        self.post("/construction/preprocess", &request).await
    }

    /// `/construction/metadata`: the sender's nonce and the suggested fee,
    /// from the options preprocess returned.
    pub async fn construction_metadata(
        &self,
        options: Option<Metadata>,
        public_keys: Vec<PublicKey>,
    ) -> Result<ConstructionMetadataResponse> {
        let request = ConstructionMetadataRequest {
            network_identifier: self.network().clone(),
            options,
            public_keys,
        };
        self.post("/construction/metadata", &request).await
    }

    /// `/construction/payloads`: the unsigned transaction and what to sign.
    pub async fn construction_payloads(
        &self,
        operations: Vec<Operation>,
        metadata: Option<Metadata>,
        public_keys: Vec<PublicKey>,
    ) -> Result<ConstructionPayloadsResponse> {
        let request = ConstructionPayloadsRequest {
            network_identifier: self.network().clone(),
            operations,
            metadata,
            public_keys,
        };
        self.post("/construction/payloads", &request).await
    }

    /// `/construction/combine`: the unsigned transaction with its signatures.
    pub async fn construction_combine(
        &self,
        unsigned_transaction: impl Into<String>,
        signatures: Vec<Signature>,
    ) -> Result<ConstructionCombineResponse> {
        let request = ConstructionCombineRequest {
            network_identifier: self.network().clone(),
            unsigned_transaction: unsigned_transaction.into(),
            signatures,
        };
        self.post("/construction/combine", &request).await
    }

    /// `/construction/parse`: the operations of a transaction, to check it
    /// before signing or submitting.
    pub async fn construction_parse(
        &self,
        signed: bool,
        transaction: impl Into<String>,
    ) -> Result<ConstructionParseResponse> {
        let request = ConstructionParseRequest {
            network_identifier: self.network().clone(),
            signed,
            transaction: transaction.into(),
        };
        self.post("/construction/parse", &request).await
    }

    /// `/construction/hash`: the hash of a signed transaction.
    pub async fn construction_hash(
        &self,
        signed_transaction: impl Into<String>,
    ) -> Result<TransactionIdentifierResponse> {
        let request = self.signed_transaction_request(signed_transaction);
        self.post("/construction/hash", &request).await
    }

    /// `/construction/submit`: sends a signed transaction to the daemon.
    pub async fn construction_submit(
        &self,
        signed_transaction: impl Into<String>,
    ) -> Result<TransactionIdentifierResponse> {
        let request = self.signed_transaction_request(signed_transaction);
        self.post("/construction/submit", &request).await
    }

    /// POSTs `body` to `endpoint` (such as `/network/status`) with the
    /// client's retries, for endpoints this crate does not wrap.
    pub async fn post<B, T>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = format!("{}{endpoint}", self.config.base_url);
        let body = serde_json::to_vec(body).map_err(|cause| Error::Encode {
            endpoint: endpoint.to_string(),
            cause,
        })?;
        let retries = self.config.retries;
        let mut attempt = 0;
        loop {
            attempt += 1;
            let last = attempt == retries;
            debug!(endpoint, attempt, retries, "Rosetta request");
            let answer = match self.http.post(&url).body(body.clone()).send().await {
                Ok(response) => {
                    let status = response.status();
                    response.text().await.map(|text| (status, text))
                }
                Err(cause) => Err(cause),
            };
            let (status, text) = match answer {
                Ok(answer) => answer,
                Err(cause) if !last => {
                    warn!(endpoint, attempt, error = %cause, "no answer, retrying");
                    tokio::time::sleep(self.config.retry_delay).await;
                    continue;
                }
                Err(cause) => {
                    return Err(Error::Connection {
                        endpoint: endpoint.to_string(),
                        attempts: attempt,
                        cause,
                    })
                }
            };
            if status.is_success() {
                return serde_json::from_str(&text).map_err(|cause| Error::Decode {
                    endpoint: endpoint.to_string(),
                    cause,
                    body: text,
                });
            }
            // A Rosetta error says whether to retry; without one, a 5xx may
            // pass and a 4xx will not.
            let api_error = serde_json::from_str::<ApiError>(&text).ok();
            let retriable = api_error
                .as_ref()
                .map_or(status.is_server_error(), |error| error.retriable);
            if last || !retriable {
                return Err(match api_error {
                    Some(error) => Error::Api {
                        endpoint: endpoint.to_string(),
                        status: status.as_u16(),
                        error,
                    },
                    None => Error::Http {
                        endpoint: endpoint.to_string(),
                        status: status.as_u16(),
                        body: text.chars().take(BODY_LIMIT).collect(),
                    },
                });
            }
            warn!(endpoint, attempt, %status, "server error, retrying");
            tokio::time::sleep(self.config.retry_delay).await;
        }
    }

    fn network_request(&self) -> NetworkRequest {
        NetworkRequest {
            network_identifier: self.network().clone(),
            metadata: None,
        }
    }

    fn signed_transaction_request(
        &self,
        signed_transaction: impl Into<String>,
    ) -> SignedTransactionRequest {
        SignedTransactionRequest {
            network_identifier: self.network().clone(),
            signed_transaction: signed_transaction.into(),
        }
    }
}
