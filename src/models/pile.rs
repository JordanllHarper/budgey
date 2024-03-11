use serde::{Deserialize, Serialize};

use super::record_transaction::{Record, Transaction, TransactionType};

#[derive(Serialize, Deserialize)]
pub struct Pile {
    pub name: String,
    pub current_balance: f32,
    pub pile_type: PileType,
    pub records: Vec<Record>,
    pub current_transactions: Vec<Transaction>,
}

impl Default for Pile {
    fn default() -> Self {
        Pile::new(
            "main",
            0.0,
            PileType::Main,
            &[Transaction::new(TransactionType::Init, 0.0)],
        )
    }
}

#[derive(Serialize, Deserialize)]
pub enum PileType {
    Main,
    UserCreated,
}

impl Pile {
    pub fn new(name: String, balance: f32, pile_type: PileType) -> Self {
    pub fn new(
        name: &str,
        balance: f32,
        pile_type: PileType,
        current_transactions: &[Transaction],
    ) -> Self {
        Self {
            name: name.to_string(),
            current_balance: balance,
            pile_type,
            records: vec![],
            current_transactions: current_transactions.to_vec(),
        }
    }
    pub fn default_main_pile() -> Pile {
        Pile::new("main".to_string(), 0.0, PileType::Main)
    }
}
