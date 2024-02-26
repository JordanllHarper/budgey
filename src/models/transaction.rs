// TODO: Add budget and pile data into a transaction so we don't nest data inside the budget and
// transaction models -> greater efficiency (and maybe paging?)
pub struct Transaction {
    pub amount: f64,
    pub source_pile: String,
    pub transaction_type: TransactionType,
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
