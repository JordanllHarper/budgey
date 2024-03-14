#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BudgetCollection {
    pub budget_names: Vec<String>,
}

impl BudgetCollection {
    pub fn new(budget_names: Vec<String>) -> Self {
        Self { budget_names }
    }

    pub fn add_name(self, name: String) -> Self {
        let budget_names = self
            .budget_names
            .into_iter()
            .chain(std::iter::once(name))
            .collect::<_>();
        BudgetCollection::new(budget_names)
    }

    pub fn new_init() -> Self {
        Self {
            budget_names: vec!["main".to_string()],
        }
    }
}
