use crate::models::budget::Budget;

trait BudgetRepository {
    fn create_new_budget(&self, budget: Budget);
    fn get_all_budgets(&self);
    fn delete_budget(&self, budget_name: &str);
}
