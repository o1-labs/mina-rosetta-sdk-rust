use crate::models::{AccountIdentifier, Amount, Operation, OperationIdentifier, OperationType};
use crate::DEFAULT_TOKEN_ID;

/// A payment of `amount` nanomina from `sender` to `receiver`, with a fee
/// of `fee` nanomina.
///
/// ```
/// use mina_rosetta_sdk::{OperationType, Transfer};
///
/// let operations = Transfer::new("B62qsender", "B62qreceiver", 1_500_000_000, 10_000_000)
///     .operations();
/// let types: Vec<_> = operations.iter().filter_map(|o| o.kind()).collect();
/// assert_eq!(
///     types,
///     [OperationType::FeePayment, OperationType::PaymentSourceDec, OperationType::PaymentReceiverInc],
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub fee: u64,
    pub token_id: String,
}

impl Transfer {
    /// A MINA payment.
    pub fn new(
        sender: impl Into<String>,
        receiver: impl Into<String>,
        amount: u64,
        fee: u64,
    ) -> Self {
        Transfer {
            sender: sender.into(),
            receiver: receiver.into(),
            amount,
            fee,
            token_id: DEFAULT_TOKEN_ID.to_string(),
        }
    }

    /// The operations of the payment, as Mina's server reads them:
    /// `fee_payment` and `payment_source_dec` on the sender, then
    /// `payment_receiver_inc` on the receiver, related to the second.
    pub fn operations(&self) -> Vec<Operation> {
        let sender = AccountIdentifier::with_token(&self.sender, &self.token_id);
        let receiver = AccountIdentifier::with_token(&self.receiver, &self.token_id);
        vec![
            Operation {
                operation_identifier: OperationIdentifier::new(0),
                operation_type: OperationType::FeePayment.to_string(),
                account: Some(sender.clone()),
                amount: Some(Amount::debit(self.fee)),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier::new(1),
                operation_type: OperationType::PaymentSourceDec.to_string(),
                account: Some(sender),
                amount: Some(Amount::debit(self.amount)),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier::new(2),
                related_operations: vec![OperationIdentifier::new(1)],
                operation_type: OperationType::PaymentReceiverInc.to_string(),
                account: Some(receiver),
                amount: Some(Amount::credit(self.amount)),
                ..Default::default()
            },
        ]
    }
}
