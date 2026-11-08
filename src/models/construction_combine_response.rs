use serde::{Deserialize, Serialize};

/// `/construction/combine`: the signed transaction.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConstructionCombineResponse {
    pub signed_transaction: String,
}
