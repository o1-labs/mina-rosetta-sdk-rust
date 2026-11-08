use serde::{Deserialize, Serialize};

/// The operation types of Mina's Rosetta server
/// (`src/lib/rosetta_lib/operation_types.ml`).
///
/// A payment is `FeePayment`, `PaymentSourceDec` and
/// `PaymentReceiverInc`; a delegation is `FeePayment` and
/// `DelegateChange`; the operation builders in [`crate::operations`] make
/// both.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    strum::IntoStaticStr,
    strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum OperationType {
    FeePayerDec,
    FeeReceiverInc,
    CoinbaseInc,
    AccountCreationFeeViaPayment,
    AccountCreationFeeViaFeeReceiver,
    PaymentSourceDec,
    PaymentReceiverInc,
    FeePayment,
    DelegateChange,
    ZkappFeePayerDec,
    ZkappBalanceUpdate,
    AccountCreationFeeViaZkapp,
}

impl OperationType {
    /// The name the server uses, such as `payment_source_dec`.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}
