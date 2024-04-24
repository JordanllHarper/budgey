use chrono::{DateTime, Local};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

/// Represents a record that has been made in a pile.
/// This is analogous to a commit in Git.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    /// The unique identifier for this record.
    pub id: String,
    /// The message for this record.
    pub message: String,
    /// The time stamp for this record. In epoch time.
    pub local_commit_time: DateTime<Local>,
    /// The amount the pile has after this record.
    pub amount_after_record: f32,
    /// The actions that have been made in this record.
    pub transactions: Vec<Transaction>,
}

/// Represents an action that has been made in a record.
/// This is analogous to a file change followed by Git add *filepath.file_type*.
/// For example, adding 10 to a pile.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    /// The type of transaction.
    pub transaction_type: TransactionType,
    /// The amount dealt with in this transaction.
    pub amount: f32,
    /// An optional note for this transaction   
    pub note: Option<String>,
}

impl Transaction {
    pub fn new(transaction_type: TransactionType, amount: f32, note: Option<&str>) -> Self {
        Self {
            transaction_type,
            amount,
            note: note.map(|s| s.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TransactionType {
    /// Adding to a pile. For example, adding 10 to a pile.
    Add,
    /// Withdrawing from a pile. For example, withdrawing 10 from a pile.
    Withdraw,
    /// Initialising a pile. This will be the first transaction in a pile.
    Init,
}

impl Record {
    pub fn new_init(message: &str, amount_after_record: f32, amount: Option<f32>) -> Self {
        Self::new(
            message,
            &chrono::Local::now(),
            amount_after_record,
            &[Transaction::new(
                TransactionType::Init,
                amount.unwrap_or(0.0),
                None,
            )],
        )
    }
    pub fn new(
        message: &str,
        time_stamp: &DateTime<chrono::Local>,
        amount_after_record: f32,
        transactions: &[Transaction],
    ) -> Self {
        Self {
            id: nanoid!().to_string(),
            message: message.to_string(),
            amount_after_record,
            transactions: transactions.to_vec(),
            local_commit_time: time_stamp.clone(),
        }
    }
}
