use crate::utils::concat_paths;

#[derive(Debug, Clone)]
pub struct BudgeyConfig {
    pub budgey_path: String,
    pub state_json_name: String,
}

impl BudgeyConfig {
    pub fn new(budgey_path: &str, state_json_name: &str) -> Self {
        Self {
            budgey_path: budgey_path.to_string(),
            state_json_name: state_json_name.to_string(),
        }
    }
    pub fn get_budget_path(&self, budget_name: &str) -> String {
        concat_paths(&self.budgey_path, budget_name)
    }
}
