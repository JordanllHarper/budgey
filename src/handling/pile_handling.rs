use std::fs::{self};

use crate::{
    models::{budget::Budget, pile::Pile},
    utils::json_utils::create_json_path,
};

use super::budget_handling::GetBudgetError;

#[derive(Debug)]
pub enum CreateNewPileError {
    SubPileError(SubPileError),
    PileDirectoryAlreadyExists,
    CouldntCreatePileDirectory,
    CouldntWriteJson,
}

#[derive(Debug)]
pub enum SubPileError {
    NoSourcePile,
}

/// Creates a new pile in the Budgey -> Budget file directory.
pub fn create_new_pile(
    pile: Pile,
    budget_name: &str,
    budgey_path: &str,
) -> anyhow::Result<(), CreateNewPileError> {
    let pile_name = &pile.name;
    let pile_path = format!("{}/{}/{}", budgey_path, budget_name, pile_name);
    fs::create_dir(&pile_path).map_err(|e| {
        if let std::io::ErrorKind::AlreadyExists = e.kind() {
            CreateNewPileError::PileDirectoryAlreadyExists
        } else {
            CreateNewPileError::CouldntCreatePileDirectory
        }
    })?;
    let json_file = create_json_path(&pile_path, pile_name);
    fs::write(json_file, serde_json::to_string(&pile).unwrap())
        .map_err(|_| CreateNewPileError::CouldntWriteJson)?;

    Ok(())
}

#[derive(Debug)]
pub enum GetPilesError {
    GetPileByNameError(GetPileByNameError),
}

/// Gets all the piles in the given budget.
pub fn get_all_piles(
    budgey_directory: &str,
    budget_name: &str,
    pile_names: &[&str],
    get_pile_by_name: fn(
        budgey_directory: &str,
        budget_name: &str,
        pile_name: &str,
    ) -> anyhow::Result<Pile, GetPileByNameError>,
) -> anyhow::Result<Vec<Pile>, GetPilesError> {
    pile_names
        .iter()
        .map(|pile_name| {
            get_pile_by_name(budgey_directory, budget_name, pile_name)
                .map_err(|e| GetPilesError::GetPileByNameError(e))
        })
        .collect::<anyhow::Result<Vec<Pile>, GetPilesError>>()
}
pub fn get_pile_json(
    budgey_path: &str,
    budget_name: &str,
    pile_name: &str,
) -> anyhow::Result<String, GetPileByNameError> {
    let pile_path = format!("{}/{}/{}", budgey_path, budget_name, pile_name);
    let pile_json = fs::read_to_string(create_json_path(&pile_path, pile_name))
        .map_err(|_| GetPileByNameError::NoPileJsonError)?;
    Ok(pile_json)
}

#[derive(Debug)]
pub enum GetPileByNameError {
    GetBudgetError(GetBudgetError),
    NoPileJsonError,
    PileDeserializationError,
    NamedPileNotInBudget,
}

/// Gets a pile by its name in the given budget.
pub fn get_pile_by_name(
    budget_name: &str,
    pile_name: &str,
    get_budget: impl Fn(&str) -> anyhow::Result<Budget, GetBudgetError>,
    get_pile_json: impl Fn(&str) -> anyhow::Result<String, GetPileByNameError>,
) -> anyhow::Result<Pile, GetPileByNameError> {
    let budget = get_budget(budget_name).map_err(|e| GetPileByNameError::GetBudgetError(e))?;
    let has_pile = budget.pile_names.iter().any(|pile| pile == pile_name);
    if !has_pile {
        return Err(GetPileByNameError::NamedPileNotInBudget);
    }
    let pile_json = get_pile_json(pile_name)?;
    let pile: Pile = serde_json::from_str(&pile_json)
        .map_err(|_| GetPileByNameError::PileDeserializationError)?;
    Ok(pile)
}
// TODO: Add variants when implementing
pub enum SwitchPileError {}
/// Switches the focused pile to the pile with the given name.
pub fn switch_pile(_pile_name: &str) -> anyhow::Result<(), SwitchPileError> {
    todo!()
}

// TODO: Add variants when implementing
pub enum DeletePileError {}
/// Deletes a pile with the given name.
pub fn delete_pile(_pile_name: &str) -> anyhow::Result<(), DeletePileError> {
    todo!()
}
