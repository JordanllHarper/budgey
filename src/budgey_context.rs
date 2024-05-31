use crate::{
    budgey_state::BudgeyState,
    utils::{concat_paths, create_json_file_name},
    BudgeyConfig,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod test {
    use crate::{budgey_config::BudgeyConfig, budgey_state::BudgeyState};

    use super::BudgeyContext;

    #[test]
    fn update_state_changes_context_state() {
        let state = BudgeyState::new(&["budget_one".to_string()], "budget_one");
        let new_state = BudgeyState::new(
            &["budget_one".to_string(), "budgey_two".to_string()],
            "budget_one",
        );
        let config = BudgeyConfig::new("test/", "path.json");
        let sample = BudgeyContext::new(&state, &config);
        let expected = BudgeyContext::new(&new_state, &config);
        let actual = sample.update_state(&new_state);
        assert_eq!(expected, actual);
    }
    #[test]
    fn get_current_budget_json_path() {
        let sample = BudgeyContext::new(
            &BudgeyState::new(&["test_budget".to_string()], "test_budget"),
            &BudgeyConfig::new("test", "state.json"),
        );

        let expected = "test/test_budget/test_budget.json";
        let actual = sample.get_current_budget_json_path();
        assert_eq!(expected, actual);
    }
    #[test]
    fn get_current_budget_path() {
        let sample = BudgeyContext::new(
            &BudgeyState::new(&[], "test_budget"),
            &BudgeyConfig::new("test", "state.json"),
        );
        let expected = "test/test_budget";
        let actual = sample.get_current_budget_path();
        assert_eq!(expected, actual);
    }
}
