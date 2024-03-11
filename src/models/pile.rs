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
        Pile::new("main", 0.0, PileType::Main)
    }
}

#[derive(Serialize, Deserialize)]
pub enum PileType {
    Main,
    UserCreated,
}

impl Pile {
    pub fn new(name: String, balance: f32, pile_type: PileType) -> Self {
        Self {
            name,
            current_balance: balance,
            pile_type,
        }
    }
    pub fn default_main_pile() -> Pile {
        Pile::new("main".to_string(), 0.0, PileType::Main)
    }
}
