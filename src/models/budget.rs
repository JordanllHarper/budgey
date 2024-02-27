use serde::{Deserialize, Serialize};

use super::pile::Pile;

/// Represents a collection of piles that a user might have
/// Stored in /budgey/[budget_name]/
#[derive(Serialize, Deserialize)]
pub struct Budget {
    repo_name: String,
    pile_list: Vec<Pile>,
}

impl Budget {
    pub fn new(budget_name: &str, pile_list: Vec<Pile>) -> Self {
        Self {
            repo_name: budget_name.to_string(),
            pile_list,
        }
    }
}
