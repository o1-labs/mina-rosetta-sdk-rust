use crate::models::ApiError;

/// Errors of [`crate::RosettaClient`].
///
/// The cause of an error is part of its message, so a field that holds it
/// is called `cause`, not `source`: thiserror would make a `source` field
/// the error's source, and a printer of the whole chain would show it twice.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The server answered with a Rosetta error.
    #[error("{endpoint}: {error}")]
    Api {
        endpoint: String,
        status: u16,
        error: ApiError,
    },

    /// The server answered with an HTTP error and no Rosetta error body.
    #[error("{endpoint}: HTTP {status}: {body}")]
    Http {
        endpoint: String,
        status: u16,
        body: String,
    },

    /// The server could not be reached, on every attempt.
    #[error("{endpoint}: no answer after {attempts} attempt(s): {cause}")]
    Connection {
        endpoint: String,
        attempts: u32,
        cause: reqwest::Error,
    },

    /// The server answered, but not with the expected response.
    #[error("{endpoint}: unexpected response: {cause}")]
    Decode {
        endpoint: String,
        cause: serde_json::Error,
        body: String,
    },

    /// The request body could not be written as JSON.
    #[error("{endpoint}: cannot write the request: {cause}")]
    Encode {
        endpoint: String,
        cause: serde_json::Error,
    },

    /// The client configuration is not usable.
    #[error("invalid client configuration: {0}")]
    Config(&'static str),

    /// The HTTP client could not be built, for example its TLS.
    #[error("cannot build the HTTP client: {cause}")]
    Client { cause: reqwest::Error },

    /// `/network/list` returned no network.
    #[error("{0} serves no network")]
    NoNetwork(String),
}

impl Error {
    /// The Rosetta error, if the server returned one.
    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            Error::Api { error, .. } => Some(error),
            _ => None,
        }
    }
}

/// The result of [`crate::RosettaClient`] calls.
pub type Result<T> = std::result::Result<T, Error>;
