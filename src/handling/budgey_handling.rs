use std::{fs, io::ErrorKind};

use crate::{models::budget_collection::BudgetCollection, utils::json_utils::create_json_path};

#[derive(Debug)]
pub enum InitBudgeyError {
    BudgeyAlreadyExists,
    BudgeyCreationFailed,
}
pub enum CheckBudgeyExistsError {
    SomethingWentWrong,
}
pub fn init_budgey(budgey_path: &str) -> anyhow::Result<(), InitBudgeyError> {
    fs::create_dir(&budgey_path).map_err(|e| {
        if let ErrorKind::AlreadyExists = e.kind() {
            InitBudgeyError::BudgeyAlreadyExists
        } else {
            InitBudgeyError::BudgeyCreationFailed
        }
    })?;
    let budget_collection_json_path = create_json_path(budgey_path, "budget_collection");
    let budgets_collection = BudgetCollection::new_init();
    Ok(())
}

use std::env;

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
