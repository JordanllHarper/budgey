use std::fmt::{Display, Formatter};

use crate::{
    cli::budgeter_cli::PileSubcommand,
    handling::{
        budget_handling,
        pile_handling::{self, CreateNewPileError, GetPileByNameError, GetPilesError},
    },
    models::pile::{Pile, PileType},
};

#[derive(Debug)]
pub enum PileError {
    CreateNewPileError(CreateNewPileError),
    GetPilesError(GetPilesError),
    PileOperationError(PileOperationError),
}
impl PileError {
    pub fn new_from_create_new_pile_error(e: CreateNewPileError) -> Self {
        PileError::CreateNewPileError(e)
    }

    pub fn new_from_pile_operation_error(e: PileOperationError) -> Self {
        PileError::PileOperationError(e)
    }
}

impl Display for PileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PileError::CreateNewPileError(e) => match e {
                CreateNewPileError::SubPileError(sub_e) => write!(f, "{:?}", sub_e),
                CreateNewPileError::PileDirectoryAlreadyExists => {
                    write!(f, "The pile directory already exists.")
                }
                CreateNewPileError::CouldntCreatePileDirectory => {
                    write!(f, "Couldn't create the pile directory.")
                }
                CreateNewPileError::CouldntWriteJson => {
                    write!(f, "Couldn't write the pile json file.")
                }
            },
            PileError::GetPilesError(e) => write!(f, "{:?}", e),
            PileError::PileOperationError(e) => write!(f, "{:?}", e),
        }
    }
}

pub fn handle_pile_command(
    budgey_path: &str,
    budget_name: &str,
    subcommand: PileSubcommand,
    get_pile_json: fn(&str, &str, &str) -> anyhow::Result<String, GetPileByNameError>,
) -> anyhow::Result<(), PileError> {
    match subcommand {
        PileSubcommand::New {
            source,
            name,
            initial_balance,
        } => {
            let source_pile: Pile = pile_handling::get_pile_by_name(
                budget_name,
                &source,
                |budget_name: &str| {
                    budget_handling::get_budget(|| {
                        budget_handling::get_budget_json(budgey_path, budget_name)
                    })
                },
                |pile_name| get_pile_json(budgey_path, budget_name, pile_name),
            )
            .map_err(|_| {
                PileError::new_from_create_new_pile_error(CreateNewPileError::SubPileError(
                    pile_handling::SubPileError::NoSourcePile,
                ))
            })?;

            pile_handling::create_new_pile(
                Pile::new(
                    &name,
                    initial_balance.unwrap_or(0.0),
                    PileType::UserCreated {
                        source_pile_name: source_pile.name,
                    },
                    source_pile.records.as_slice(),
                ),
                &budget_name,
                budgey_path,
                |path| std::fs::create_dir(path),
            )
            .map_err(|e| PileError::new_from_create_new_pile_error(e))
        }
        _ => todo!(),
    }
}

#[derive(Debug)]
pub enum PileOperationError {
    MergeError,
    BalanceError,
    ListError,
    ViewError,
    RemoveError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_pile_command() {}
}
