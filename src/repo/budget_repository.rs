use std::{fs, io::ErrorKind};

use crate::{
    io_operations::io_operations::io_operations::create_json_path, models::budget::Budget,
};

#[derive(Debug)]
pub enum CreateNewBudgetError {
    CreateNamedBudgetDirFailed,
    BudgetDirectoryAlreadyExists,
    CouldntConvertToJson,
    CouldntWriteJson,
}

pub enum BudgetRepositoryError {
    CreateNewBudgetError(CreateNewBudgetError),
}

pub fn create_new_budget(
    budgey_directory: &str,
    budget: Budget,
) -> Result<(), CreateNewBudgetError> {
    let budget_name = &budget.budget_detail.budget_name;
    let budget_path = format!("{}/{}", budgey_directory, budget_name);
    fs::create_dir(&budget_path).map_err(|e| {
        if let ErrorKind::AlreadyExists = e.kind() {
            CreateNewBudgetError::BudgetDirectoryAlreadyExists
        } else {
            CreateNewBudgetError::CreateNamedBudgetDirFailed
        }
    })?;
    let json_file = create_json_path(&budget_path, budget_name);
    fs::write(json_file, serde_json::to_string(&budget).unwrap())
        .map_err(|_| CreateNewBudgetError::CouldntWriteJson)?;
    Ok(())
}
pub fn get_all_budgets() -> anyhow::Result<Vec<String>, BudgetRepositoryError> {
    todo!()
}
pub fn delete_budget(budget_name: &str) -> anyhow::Result<(), BudgetRepositoryError> {
    todo!()
}
