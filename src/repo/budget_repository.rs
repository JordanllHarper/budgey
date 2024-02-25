use crate::{config::local_config::LocalConfig, models::budget::Budget};

pub trait BudgetRepository {
    fn create_new_budget(&self, budget: Budget);
    fn get_all_budgets(&self);
    fn delete_budget(&self, budget_name: &str);
}

pub struct BudgetRepositoryImpl {
    budgey_directory: String,
}

impl BudgetRepositoryImpl {
    pub fn new(budgey_directory: &str) -> Self {
        Self {
            budgey_directory: budgey_directory.to_string(),
        }
    }
}

impl BudgetRepository for BudgetRepositoryImpl {
    fn create_new_budget(&self, budget: Budget) {}

    fn get_all_budgets(&self) {
        todo!()
    }

    fn delete_budget(&self, budget_name: &str) {
        todo!()
    }
}
