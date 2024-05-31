use crate::{
    budgey_state::BudgeyState,
    utils::{concat_paths, create_json_file_name},
    BudgeyConfig,
};

#[derive(Debug, Clone)]
pub struct BudgeyContext {
    pub budgey_config: BudgeyConfig,
    pub state: BudgeyState,
}
impl BudgeyContext {
    pub fn new(state: &BudgeyState, budgey_config: &BudgeyConfig) -> Self {
        Self {
            budgey_config: budgey_config.clone(),
            state: state.clone(),
        }
    }
    pub fn get_current_budget_path(&self) -> String {
        let current_budget = &self.state.current_focused_budget_name;
        let budgey_path = &self.budgey_config.budgey_path;
        concat_paths(budgey_path, current_budget)
    }
    pub fn get_current_budget_json_path(&self) -> String {
        concat_paths(
            &self.get_current_budget_path(),
            &create_json_file_name(&self.state.current_focused_budget_name),
        )
    }

    pub fn update_state(&self, new_state: &BudgeyState) -> Self {
        Self::new(new_state, &self.budgey_config)
    }
}
