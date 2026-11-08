use crate::models::{
    AccountIdentifier, Amount, Metadata, Operation, OperationIdentifier, OperationType,
};
use crate::DEFAULT_TOKEN_ID;
use serde_json::Value;

/// A delegation of `delegator`'s stake to `new_delegate`, with a fee of
/// `fee` nanomina.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delegation {
    pub delegator: String,
    pub new_delegate: String,
    pub fee: u64,
}

impl Delegation {
    pub fn new(delegator: impl Into<String>, new_delegate: impl Into<String>, fee: u64) -> Self {
        Delegation {
            delegator: delegator.into(),
            new_delegate: new_delegate.into(),
            fee,
        }
    }

    /// The operations of the delegation, as Mina's server reads them:
    /// `fee_payment` and `delegate_change` on the delegator, the second
    /// with the new delegate in its metadata.
    pub fn operations(&self) -> Vec<Operation> {
        let delegator = AccountIdentifier::with_token(&self.delegator, DEFAULT_TOKEN_ID);
        let mut target = Metadata::new();
        target.insert(
            "delegate_change_target".to_string(),
            Value::String(self.new_delegate.clone()),
        );
        vec![
            Operation {
                operation_identifier: OperationIdentifier::new(0),
                operation_type: OperationType::FeePayment.to_string(),
                account: Some(delegator.clone()),
                amount: Some(Amount::debit(self.fee)),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier::new(1),
                operation_type: OperationType::DelegateChange.to_string(),
                account: Some(delegator),
                metadata: Some(target),
                ..Default::default()
            },
        ]
    }
}
