#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BudgeyState {
    pub current_focused_budget: String,
    pub budget_names: Vec<String>,
}

impl BudgeyState {
    pub fn new(budget_names: &[String], current_focused_budget: &str) -> Self {
        Self {
            budget_names: budget_names.to_vec(),
            current_focused_budget: current_focused_budget.to_string(),
        }
    }

    pub fn add_name(self, name: String) -> Self {
        let budget_names = self
            .budget_names
            .into_iter()
            .chain(std::iter::once(name))
            .collect::<Vec<String>>();
        BudgeyState::new(&budget_names, &self.current_focused_budget)
    }

    pub fn new_init() -> Self {
        Self {
            budget_names: vec!["main".to_string()],
            current_focused_budget: "main".to_string(),
        }
    }
}
