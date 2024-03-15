use std::{fs, io::ErrorKind};

use crate::{
    models::{self, budgey_state},
    utils::json_utils::create_json_path,
};

#[derive(Debug)]
pub enum InitBudgeyError {
    BudgeyAlreadyExists,
    BudgeyCreationFailed,
}
pub enum CheckBudgeyExistsError {
    SomethingWentWrong,
}
pub fn init_budgey(
    budgey_path: &str,
    budget_state_name: &str,
) -> anyhow::Result<(), InitBudgeyError> {
    fs::create_dir(&budgey_path).map_err(|e| {
        if let ErrorKind::AlreadyExists = e.kind() {
            InitBudgeyError::BudgeyAlreadyExists
        } else {
            InitBudgeyError::BudgeyCreationFailed
        }
    })?;
    let init_state = models::budgey_state::BudgeyState::new_init();
    write_new_budgey_state(budgey_path, budgey_state_name, init_state);
    todo!()
}
#[derive(Debug)]
pub enum GetBudgeyStateError {
    BudgeyStateFileNotFound,
    ErrorReadingBudgeyState,
    DeserialiseBudgeyStateError,
}
pub fn get_budgey_state(
    budgey_directory: &str,
    budget_state_name: &str,
) -> anyhow::Result<models::budgey_state::BudgeyState, GetBudgeyStateError> {
    let budgey_state_path = create_json_path(budgey_directory, &budget_state_name);
    let result = fs::read_to_string(budgey_state_path).map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
            GetBudgeyStateError::BudgeyStateFileNotFound
        } else {
            GetBudgeyStateError::ErrorReadingBudgeyState
        }
    })?;
    let budgey_state: budgey_state::BudgeyState = serde_json::from_str(&result)
        .map_err(|_| GetBudgeyStateError::DeserialiseBudgeyStateError)?;
    Ok(budgey_state)
}

#[derive(Debug)]
pub struct LocalConfig {
    /// The path to the /budgey directory  
    pub budgey_dir: String,
}

impl Default for LocalConfig {
    fn default() -> Self {
        let budgey_dir = env!("HOME").to_string() + "/budgey";
        LocalConfig { budgey_dir }
    }
}
pub fn get_local_config() -> LocalConfig {
    todo!()
}
