use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub amount: f64,
    pub source_pile: String,
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize)]
pub enum TransactionType {
    Pull,
    Push,
    Merge {
        destination_pile: String,
        delete_after_merge: bool,
    },
}

impl Transaction {
    pub fn new(amount: f64, source_pile: String, transaction_type: TransactionType) -> Self {
        Self {
            amount,
            source_pile,
            transaction_type,
        }
    }

    pub fn new_push(amount: f64, source_pile: String) -> Self {
        Self::new(amount, source_pile, TransactionType::Push)
    }

    pub fn new_pull(amount: f64, source_pile: String) -> Self {
        Self::new(amount, source_pile, TransactionType::Pull)
    }
}
