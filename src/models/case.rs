use serde::{Deserialize, Serialize};

/// How the server writes hashes, so that a client compares them the same
/// way. Mina's hashes are base58 and `CaseSensitive`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Case {
    UpperCase,
    LowerCase,
    CaseSensitive,
    /// The string `"null"`, which the spec's generated models list as a
    /// value of its own.
    Null,
}
