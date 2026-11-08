use super::{Currency, Metadata};
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

/// A signed amount in the currency's smallest unit (nanomina for MINA), as
/// a decimal string: negative is a debit.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: Currency,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl Amount {
    /// `nanomina` MINA added to an account.
    pub fn credit(nanomina: u64) -> Self {
        Amount {
            value: nanomina.to_string(),
            currency: Currency::mina(),
            metadata: None,
        }
    }

    /// `nanomina` MINA taken from an account.
    pub fn debit(nanomina: u64) -> Self {
        Amount {
            value: format!("-{nanomina}"),
            currency: Currency::mina(),
            metadata: None,
        }
    }

    /// The value as a number.
    pub fn signed_value(&self) -> Result<i128, ParseIntError> {
        self.value.parse()
    }
}
