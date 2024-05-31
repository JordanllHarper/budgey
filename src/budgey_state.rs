use std::{fs, io::ErrorKind};

use colored::Colorize;
use log::{error, trace};

use crate::{budgey_config::BudgeyConfig, utils::concat_paths};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
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

/// Gets the budgey state from the given path
pub fn get_budgey_state(budgey_path_to_json: &str) -> anyhow::Result<BudgeyState, std::io::Error> {
    trace!("Getting budgey state");
    let read_result = match fs::read_to_string(budgey_path_to_json) {
        Ok(v) => v,
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                println!(
                    "Couldn't find the state file.\n\nPlease run {}",
                    "`budgey init`".green()
                );
                return Err(e);
            } else {
                error!("Error reading budgey state: {:?}", e);
                return Err(e);
            }
        }
    };
    let state: BudgeyState = serde_json::from_str(&read_result)?;
    Ok(state)
}

/// Checks if the budgey state has already been initialised
pub fn check_budgey_state_initialised(budgey_config: &BudgeyConfig) -> anyhow::Result<bool> {
    trace!("Checking if budgey state is initialised");
    let check_path_result = match fs::read_dir(&budgey_config.budgey_path) {
        Ok(r) => r,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Ok(false);
            }
            error!("Error reading budgey path: {:?}", e);
            return Err(e.into());
        }
    };

    let contains_result = check_path_result
        .into_iter()
        .flatten()
        .flat_map(|file| file.file_name().into_string())
        .any(|each| each.eq(&budgey_config.state_json_name));

    trace!("Budgey initialised: {}", contains_result);

    Ok(contains_result)
}

/// Writes a new budgey state file with the given path and state
pub fn write_budgey_state(
    budgey_config: &BudgeyConfig,
    new_state: &BudgeyState,
) -> anyhow::Result<(), std::io::Error> {
    trace!("Writing budgey state");
    let serialized = serde_json::to_string(new_state)?;
    let check_path_result = fs::read_dir(&budgey_config.budgey_path);
    if let Err(e) = check_path_result {
        if e.kind() == std::io::ErrorKind::NotFound {
            fs::create_dir_all(&budgey_config.budgey_path)?;
        }
    };
    fs::write(
        concat_paths(&budgey_config.budgey_path, &budgey_config.state_json_name),
        serialized,
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::BudgeyState;

    fn sample_data() -> BudgeyState {
        BudgeyState::new(
            &["budget_one".to_string(), "budget_two".to_string()],
            "budget_one",
        )
    }

    #[test]
    fn budgey_state_add_budget_name_to_state() {
        let state = sample_data();
        let expected = BudgeyState::new(
            &[
                "budget_one".to_string(),
                "budget_two".to_string(),
                "budget_three".to_string(),
            ],
            "budget_one",
        );
        let actual = state.add_budget_name("budget_three");

        assert_eq!(expected, actual);
    }
    #[test]
    fn budgey_state_change_budget_name_in_state() {
        let state = sample_data();
        let expected = BudgeyState::new(
            &["budget_one".to_string(), "budget_two".to_string()],
            "budget_two",
        );
        let actual = state.change_focused_budget_name("budget_two");

        assert_eq!(expected, actual);
    }
    #[test]
    fn budgey_state_remove_budget_name_in_state() {
        let state = sample_data();
        let expected = BudgeyState::new(&["budget_one".to_string()], "budget_one");
        let actual = state.remove_budget_name("budget_two");

        assert_eq!(expected, actual);
    }
    #[test]
    fn budgey_state_new_init_creates_new_budget_and_adds_to_list() {
        let expected = BudgeyState::new(&["budget_one".to_string()], "budget_one");
        let actual = BudgeyState::new_init("budget_one");

        assert_eq!(expected, actual);
    }
}
