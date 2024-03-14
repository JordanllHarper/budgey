use serde::{Deserialize, Serialize};

use super::record_transaction::{Record, Transaction};

#[derive(Serialize, Deserialize)]
pub struct Pile {
    pub name: String,
    pub current_balance: f32,
    pub pile_type: PileType,
    pub records: Vec<Record>,
    pub current_staged_transactions: Vec<Transaction>,
}

impl Default for Pile {
    fn default() -> Self {
        Pile::new(
            "main",
            0.0,
            PileType::Main,
            &[Record::new_init("Initialised main", "0", 0.0, Some(0.0))],
        )
    }
}

#[derive(Serialize, Deserialize)]
pub enum PileType {
    Main,
    UserCreated { source_pile_name: String },
}

impl Pile {
    pub fn new(
        name: &str,
        balance: f32,
        pile_type: PileType,
        source_record_history: &[Record],
    ) -> Self {
        let up_to_date_history = vec![Record::new_init(
            "Initialised pile",
            "0",
            balance,
            Some(balance),
        )]
        .iter()
        .chain(source_record_history)
        .cloned()
        .collect::<Vec<Record>>();
        Self {
            name: name.to_string(),
            current_balance: balance,
            pile_type,
            records: up_to_date_history,
            current_staged_transactions: vec![],
        }
    }
    pub fn default_main_pile() -> Pile {
        Pile::new(
            "main",
            0.0,
            PileType::Main,
            &[Record::new_init("Initialised main", "0", 0.0, Some(0.0))],
        )
    }
}
