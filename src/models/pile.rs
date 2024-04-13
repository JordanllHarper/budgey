use serde::{Deserialize, Serialize};

use super::record_transaction::{Record, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pile {
    pub current_balance: f32,
    pub pile_type: PileType,
    pub records: Vec<Record>,
    pub current_staged_transactions: Vec<Transaction>,
}

impl Default for Pile {
    fn default() -> Self {
        Pile::new(
            0.0,
            &PileType::Main,
            &[Record::new_init("Initialised main", "0", 0.0, Some(0.0))],
            &[],
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PileType {
    #[serde(rename = "main")]
    Main,
    #[serde(rename = "user_created")]
    UserCreated { pile_name: String },
}

impl Pile {
    pub fn get_name(&self) -> String {
        match &self.pile_type {
            PileType::Main => "main".to_string(),
            PileType::UserCreated { pile_name } => pile_name.to_string(),
        }
    }
    pub fn add_transaction(self, transaction: &Transaction) -> Self {
        Self::new(
            match transaction.transaction_type {
                super::record_transaction::TransactionType::Add => {
                    self.current_balance + transaction.amount
                }
                super::record_transaction::TransactionType::Withdraw => {
                    self.current_balance - transaction.amount
                }
                super::record_transaction::TransactionType::Init => self.current_balance,
            },
            &self.pile_type,
            &self.records,
            &self
                .clone()
                .current_staged_transactions
                .into_iter()
                .chain(vec![transaction.clone()])
                .collect::<Vec<Transaction>>()
                .as_slice(),
        )
    }
    pub fn add_record(self, record: &Record) -> Self {
        Self::new(
            self.current_balance,
            &self.pile_type,
            self.records
                .into_iter()
                .chain(vec![record.clone()])
                .collect::<Vec<Record>>()
                .as_slice(),
            &self.current_staged_transactions,
        )
    }
    pub fn clear_staged_transactions(self) -> Self {
        Self::new(self.current_balance, &self.pile_type, &self.records, &[])
    }
    pub fn new_user_created(
        balance: f32,
        pile_name: &str,
        source_record_history: &[Record],
    ) -> Self {
        Self::new(
            balance,
            &PileType::UserCreated {
                pile_name: pile_name.to_string(),
            },
            &source_record_history,
            &[],
        )
    }
    pub fn new(
        balance: f32,
        pile_type: &PileType,
        source_record_history: &[Record],
        transactions: &[Transaction],
    ) -> Self {
        let up_to_date_history = source_record_history
            .iter()
            .cloned()
            .collect::<Vec<Record>>();
        Self {
            current_balance: balance,
            pile_type: pile_type.clone(),
            records: up_to_date_history,
            current_staged_transactions: transactions.to_vec(),
        }
    }
    pub fn default_main_pile() -> Pile {
        Pile::new(
            0.0,
            &PileType::Main,
            &[Record::new_init("Initialised main", "0", 0.0, Some(0.0))],
            &[],
        )
    }
}
