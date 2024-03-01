use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use super::pile::Pile;

#[derive(Serialize, Deserialize)]
pub struct BudgetDetail {
    id: String,
    pub budget_name: String,
}

impl BudgetDetail {
    fn new(id: String, budget_name: String) -> Self {
        Self { id, budget_name }
    }
}

/// Represents a collection of piles that a user might have
/// Stored in /budgey/[budget_name]/
#[derive(Serialize, Deserialize)]
pub struct Budget {
    pub budget_detail: BudgetDetail,
    pub pile_list: Vec<Pile>,
}

impl Budget {
    pub fn new(budget_name: &str) -> Self {
        Self {
            budget_detail: BudgetDetail::new(nanoid!(), budget_name.to_string()),
            pile_list: vec![Pile::default_main_pile()],
        }
    }
}
