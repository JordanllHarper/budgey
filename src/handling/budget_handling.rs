use std::{
    fs::{self},
    io::ErrorKind,
};

use crate::{
    handling::budgey_handling::get_budgey_state,
    models::{budget::Budget, budgey_state::BudgeyState},
    utils::json_utils::create_json_path,
};

use super::budgey_handling::GetBudgeyStateError;

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
pub enum DeleteBudgetDirectoryError {
    CouldntDeleteBudgetDirectory,
}
pub fn delete_budget_directory(
    budgey_path: &str,
    budget_name: &str,
) -> anyhow::Result<(), DeleteBudgetDirectoryError> {
    let budget_path = format!("{}/{}", budgey_path, budget_name);
    fs::remove_dir_all(budget_path)
        .map_err(|_| DeleteBudgetDirectoryError::CouldntDeleteBudgetDirectory)?;
    Ok(())
}

pub enum DeleteBudgetError {
    BudgetDoesntExist,
    GetBudgeyStateError(GetBudgeyStateError),
    WriteBudgeyStateError(super::budgey_handling::WriteBudgeyStateError),
    DeleteBudgetDirectoryError(DeleteBudgetDirectoryError),
}
pub fn delete_budget(
    get_budgey_state: impl Fn() -> anyhow::Result<BudgeyState, GetBudgeyStateError>,
    budget_name: &str,
    write_new_budgey_state: impl Fn(
        BudgeyState,
    )
        -> anyhow::Result<(), super::budgey_handling::WriteBudgeyStateError>,
    delete_budget_dir: impl Fn() -> anyhow::Result<(), DeleteBudgetDirectoryError>,
) -> anyhow::Result<(), DeleteBudgetError> {
    let budgey_state = get_budgey_state().map_err(|e| DeleteBudgetError::GetBudgeyStateError(e))?;
    let budget_exists = budgey_state
        .budget_names
        .iter()
        .any(|name| name == budget_name);

    if !budget_exists {
        return Err(DeleteBudgetError::BudgetDoesntExist);
    }

    let new_budget_state = budgey_state.remove_budget_name(budget_name);

    write_new_budgey_state(new_budget_state)
        .map_err(|e| DeleteBudgetError::WriteBudgeyStateError(e))?;
    delete_budget_dir().map_err(|e| DeleteBudgetError::DeleteBudgetDirectoryError(e))?;

    Ok(())
}

pub enum SwitchBudgetError {
    AlreadyInBudgetError,
    BudgetDoesntExistError,
    GetBudgeyStateError(GetBudgeyStateError),
    WriteBudgeyStateError(super::budgey_handling::WriteBudgeyStateError),
}

pub fn switch_budget(
    get_budgey_state: impl Fn() -> anyhow::Result<BudgeyState, GetBudgeyStateError>,
    new_budget_name: &str,
    write_new_budgey_state: impl Fn(
        BudgeyState,
    )
        -> anyhow::Result<(), super::budgey_handling::WriteBudgeyStateError>,
) -> anyhow::Result<(), SwitchBudgetError> {
    let budgey_state = get_budgey_state().map_err(|e| SwitchBudgetError::GetBudgeyStateError(e))?;

    let already_in_budget = budgey_state.current_focused_budget == new_budget_name;

    if already_in_budget {
        return Err(SwitchBudgetError::AlreadyInBudgetError);
    }

    if budgey_state
        .budget_names
        .iter()
        .any(|name| name != new_budget_name)
    {
        return Err(SwitchBudgetError::BudgetDoesntExistError);
    }
    let new_state = budgey_state.change_focused_budget_name(new_budget_name);
    write_new_budgey_state(new_state).map_err(|e| SwitchBudgetError::WriteBudgeyStateError(e))?;
    Ok(())
}
