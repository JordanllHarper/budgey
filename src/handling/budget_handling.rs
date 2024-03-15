use std::{fs, io::ErrorKind};

use crate::{models::budget::Budget, utils::json_utils::create_json_path};


pub enum BudgetError {
    GetBudgetError(GetBudgetError),
    CreateNewBudgetError(CreateNewBudgetError),
}

#[derive(Debug)]
pub enum GetBudgetError {
    GetBudgetFileError,
    DeserialieBudgetJsonError,
}
pub fn get_budget(budgey_path: &str, budget_name: &str) -> anyhow::Result<Budget, BudgetError> {
    let budget_file_path = format!("{}/{}", budgey_path, budget_name);
    let budget_path = create_json_path(&budget_file_path, budget_name);
    let budget_json = fs::read_to_string(budget_path)
        .map_err(|_| BudgetError::GetBudgetError(GetBudgetError::GetBudgetFileError))?;
    let budget: Budget = serde_json::from_str(&budget_json)
        .map_err(|_| BudgetError::GetBudgetError(GetBudgetError::DeserialieBudgetJsonError))?;
    Ok(budget)
}

#[derive(Debug)]
pub enum CreateNewBudgetError {
    CreateNamedBudgetDirFailed,
    BudgetDirectoryAlreadyExists,
    CouldntConvertToJson,
    CouldntWriteJson,
}

pub fn create_new_budget(budgey_path: &str, budget: Budget) -> Result<(), CreateNewBudgetError> {
    let budget_name = &budget.budget_detail.budget_name;
    let budget_path = format!("{}/{}", budgey_path, budget_name);
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
#[derive(Debug)]
pub enum GetAllBudgetsError {
    GetBudgeyStateError(GetBudgeyStateError),
    BudgeyDirNotFound,
    ReadDirError,
    GetBudgetsFromNamesError,
}

pub fn get_all_budgets(
    budgey_path: &str,
    budget_state_name: &str,
) -> anyhow::Result<Vec<Budget>, GetAllBudgetsError> {
    let budgey_state = get_budgey_state(budgey_path, budget_state_name)
        .map_err(|e| GetAllBudgetsError::GetBudgeyStateError(e))?;
    budgey_state
        .budget_names
        .iter()
        .map(|budget_name| {
            get_budget(budgey_path, budget_name)
                .map_err(|_| GetAllBudgetsError::GetBudgetsFromNamesError)
        })
        .collect::<anyhow::Result<Vec<Budget>, GetAllBudgetsError>>()
}

pub fn delete_budget(budget_name: &str) -> anyhow::Result<(), BudgetError> {
    todo!()
}
