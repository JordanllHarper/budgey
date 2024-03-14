use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use super::pile::Pile;

#[derive(Serialize, Deserialize)]
pub struct BudgetDetail {
    id: String,
    pub budget_name: String,
}

impl BudgetDetail {
    fn new(id: &str, budget_name: &str) -> Self {
        Self {
            id: id.to_string(),
            budget_name: budget_name.to_string(),
        }
    }
}

/// Represents a collection of piles that a user might have
/// Stored in /budgey/[budget_name]/
#[derive(Serialize, Deserialize)]
pub struct Budget {
    pub budget_detail: BudgetDetail,
    pub pile_names: Vec<String>,
}

impl Budget {
    pub fn new(budget_name: &str) -> Self {
        Self {
            budget_detail: BudgetDetail::new(&nanoid!(), budget_name),
            pile_names: vec!["main".to_string()],
        }
    }
}
