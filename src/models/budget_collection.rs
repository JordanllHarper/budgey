#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BudgetCollection {
    pub budget_names: Vec<String>,
}
