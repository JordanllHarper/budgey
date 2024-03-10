use serde::{Deserialize, Serialize};

use super::transaction::Transaction;

#[derive(Serialize, Deserialize)]
pub struct Pile {
    pub name: String,
    pub balance: f32,
    pub pile_type: PileType,
    pub pile_transaction_history: Vec<Transaction>,
}

impl Default for Pile {
    fn default() -> Self {
        Pile::new("main".to_string(), 0.0, PileType::Main, vec![])
    }
}

#[derive(Serialize, Deserialize)]
pub enum PileType {
    Main,
    UserCreated,
}

impl Pile {
    pub fn new(
        name: String,
        balance: f32,
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
    pub fn default_main_pile() -> Pile {
        Pile::new("main".to_string(), 0.0, PileType::Main, vec![])
    }
}
