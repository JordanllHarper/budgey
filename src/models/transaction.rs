pub struct Transaction {
    amount: f64,
    source_pile: String,
    transaction_type: TransactionType,
}

pub enum TransactionType {
    Pull,
    Push,
    Merge {
        destination_pile: String,
        delete_after_merge: bool,
    },
}

impl Transaction {
    fn new(amount: f64, source_pile: String, transaction_type: TransactionType) -> Self {
        Self {
            amount,
            source_pile,
            transaction_type,
        }
    }

    fn new_push(amount: f64, source_pile: String) -> Self {
        Self::new(amount, source_pile, TransactionType::Push)
    }

    fn new_pull(amount: f64, source_pile: String) -> Self {
        Self::new(amount, source_pile, TransactionType::Pull)
    }
}