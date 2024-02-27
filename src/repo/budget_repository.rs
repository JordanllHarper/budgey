use crate::models::budget::Budget;

pub trait BudgetRepository {
    fn create_new_budget(&self, budget: Budget) -> anyhow::Result<String>;
    fn get_all_budgets(&self) -> anyhow::Result<String>;
    fn delete_budget(&self, budget_name: &str) -> anyhow::Result<String>;
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
    fn create_new_budget(&self, budget: Budget) -> anyhow::Result<String> {
        let budget_name = &budget.budget_detail.budget_name;
        let detail_json = serde_json::to_string(&budget.budget_detail)?;
        todo!()
    }

    fn get_all_budgets(&self) -> anyhow::Result<String> {
        todo!()
    }

    fn delete_budget(&self, budget_name: &str) -> anyhow::Result<String> {
        todo!()
    }
}
