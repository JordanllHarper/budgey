use serde::{Deserialize, Serialize};

/// Represents a transaction that has been made in a pile.
/// This is analogous to a commit in Git.
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    /// The unique identifier for this transaction.
    pub id: String,
    /// The message for this transaction.
    pub message: String,
    /// The time stamp for this transaction.
    pub time_stamp: String,
    /// The amount the pile has after this transaction.
    pub amount_after_transaction: f32,
    /// The actions that have been made in this transaction.
    pub actions: Vec<Action>,
}

/// Represents an action that has been made in a transaction.
/// This is analogous to a file change followed by Git add *filepath.file_type*.
/// For example, adding 10 of whatever currency you're using to a pile.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Action {
    /// The type of action.
    pub action_type: ActionType,
    /// The amount dealt with in this action.
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActionType {
    /// Adding to a pile. For example, adding 10 to a pile.
    Add,
    /// Withdrawing from a pile. For example, withdrawing 10 from a pile.
    Withdraw,
}

impl Transaction {
    pub fn new(
        id: &str,
        message: &str,
        time_stamp: &str,
        amount_after_transaction: f32,
        actions: &[Action],
    ) -> Self {
        Self {
            id: id.to_string(),
            message: message.to_string(),
            amount_after_transaction,
            actions: actions.to_vec(),
            time_stamp: time_stamp.to_string(),
        }
    }
}
