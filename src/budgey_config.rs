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

#[cfg(test)]
mod test {
    use super::BudgeyConfig;

    #[test]
    fn get_budget_path_gives_custom_path() {
        let sample = BudgeyConfig::new("test", "path.json");
        let expected = "test/path.json";
        let actual = sample.get_budget_path("path.json");
        assert_eq!(expected, actual);
    }
}
