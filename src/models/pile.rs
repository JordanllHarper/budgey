use super::transaction::Transaction;

pub struct Pile {
    pub name: String,
    pub balance: f64,
    pub pile_type: PileType,
    pub pile_transaction_history: Vec<Transaction>,
}

pub enum PileType {
    Main,
    UserCreated,
}

impl Pile {
    pub fn new(
        name: String,
        balance: f64,
        pile_type: PileType,
        transaction_history: Vec<Transaction>,
    ) -> Self {
        Self {
            name,
            balance,
            pile_type,
            pile_transaction_history: transaction_history,
        }
    }
}
