use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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
    pub current_pile_name: String,
}

impl Budget {
    pub fn new(
        budget_detail: BudgetDetail,
        pile_names: Vec<String>,
        current_pile_name: String,
    ) -> Self {
        Self {
            budget_detail,
            pile_names,
            current_pile_name,
        }
    }

    pub fn new_init(budget_name: &str) -> Self {
        Budget::new(
            BudgetDetail::new(&nanoid!(), budget_name),
            vec!["main".to_string()],
            "main".to_string(),
        )
    }
    pub fn change_pile_name(&self, name: &str) -> Self {
        Budget::new(
            self.budget_detail.clone(),
            self.pile_names.clone(),
            name.to_string(),
        )
    }
    pub fn add_pile(&self, pile_name: &str) -> Self {
        let new_pile_names = self
            .pile_names
            .clone()
            .into_iter()
            .chain(vec![pile_name.to_string()])
            .collect::<Vec<String>>();
        Budget::new(
            self.budget_detail.clone(),
            new_pile_names,
            self.current_pile_name.clone(),
        )
    }
    pub fn delete_pile(&self, pile_name: &str) -> Self {
        let new_pile_names = self
            .pile_names
            .clone()
            .into_iter()
            .filter(|name| name != pile_name)
            .collect::<Vec<String>>();
        Budget::new(
            self.budget_detail.clone(),
            new_pile_names,
            self.current_pile_name.clone(),
        )
    }
}
