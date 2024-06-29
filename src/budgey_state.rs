use std::fs;

use crate::{utils::concat_paths, BudgeyConfig};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct BudgeyState {
    pub current_focused_budget_name: String,
    pub budget_names: Vec<String>,
}

impl BudgeyState {
    pub fn new(budget_names: &[String], current_focused_budget: &str) -> Self {
        Self {
            budget_names: budget_names.to_vec(),
            current_focused_budget_name: current_focused_budget.to_string(),
        }
    }

    pub fn add_budget_name(&self, budget_name: &str) -> Self {
        let budget_names = self
            .clone()
            .budget_names
            .into_iter()
            .chain(std::iter::once(budget_name.to_string()))
            .collect::<Vec<String>>();
        BudgeyState::new(&budget_names, &self.current_focused_budget_name)
    }
    pub fn change_focused_budget_name(&self, budget_name: &str) -> Self {
        BudgeyState::new(&self.budget_names, budget_name)
    }

    pub fn remove_budget_name(&self, budget_name: &str) -> Self {
        let budget_names = self
            .clone()
            .budget_names
            .into_iter()
            .filter(|name| budget_name != name)
            .collect::<Vec<String>>();
        BudgeyState::new(&budget_names, &self.current_focused_budget_name)
    }
    pub fn new_init(budget_name: &str) -> Self {
        Self {
            budget_names: vec![budget_name.to_string()],
            current_focused_budget_name: budget_name.to_string(),
        }
    }
}

/// Writes a new budgey state file with the given path and state
pub fn write_budgey_state(
    budgey_config: &BudgeyConfig,
    new_state: &BudgeyState,
) -> anyhow::Result<(), std::io::Error> {
    let serialized = serde_json::to_string(new_state)?;
    let check_path_result = fs::read_dir(&budgey_config.root_path);
    if let Err(e) = check_path_result {
        if e.kind() == std::io::ErrorKind::NotFound {
            fs::create_dir_all(&budgey_config.root_path)?;
        }
    };
    fs::write(
        concat_paths(&budgey_config.root_path, &budgey_config.state_json_name),
        serialized,
    )?;

    Ok(())
}
