use crate::{config::local_config::LocalConfig, models::budget::Budget};

trait BudgetRepository {
    fn create_new_budget(&self, budget: Budget);
    fn get_all_budgets(&self);
    fn delete_budget(&self, budget_name: &str);
}

struct BudgetRepositoryImpl {
    local_config: LocalConfig,
}

impl BudgetRepositoryImpl {
    fn new(local_config: LocalConfig) -> Self {
        Self { local_config }
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
