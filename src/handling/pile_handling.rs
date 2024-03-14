use std::fs::{self};

use crate::{models::pile::Pile, utils::json_utils::create_json_path};

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
    NoBudgetDirectory,
    NamedPileNotInBudget,
    ReadBudgetError,
    DeserializeBudgetError,
    NoPilesError,
}

/// Gets all the piles in the given budget.
fn get_all_piles(
    budgey_directory: &str,
    budget_name: &str,
) -> anyhow::Result<Vec<Pile>, GetPilesError> {
    let budget_directory = format!("{}/{}", budgey_directory, budget_name);
    let read_dir = fs::read_dir(budget_directory).map_err(|_| GetPilesError::NoBudgetDirectory)?;
    todo!()
}
fn get_pile_by_name(name  :&str) -> anyhow::Result<Pile, GetPilesError> {

    todo!()
} 
/// Switches the focused pile to the pile with the given name.
fn switch_pile(pile_name: &str) -> anyhow::Result<(), CreateNewPileError> {
    todo!()
}

/// Deletes a pile with the given name.
fn delete_pile(pile_name: &str) -> anyhow::Result<(), CreateNewPileError> {
    todo!()
}
