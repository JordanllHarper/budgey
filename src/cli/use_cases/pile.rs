use std::fmt::{Display, Formatter};

use crate::{
    cli::budgeter_cli::PileSubcommand,
    handling::pile_handling::{self, CreateNewPileError, GetPilesError},
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
        todo!()
    }
}

pub fn handle_pile_command(
    budgey_path: &str,
    budget_name: &str,
    subcommand: PileSubcommand,
) -> anyhow::Result<(), PileError> {
    match subcommand {
        PileSubcommand::New {
            source,
            name,
            initial_balance,
        } => {
            let source_pile: Pile = pile_handling::get_pile_by_name(
                budgey_path,
                budget_name,
                &source,
                budget_handling::get_budget,
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
            )
            .map_err(|e| PileError::new_from_create_new_pile_error(e))
        }
        PileSubcommand::Add { amount, source } => todo!(),
        PileSubcommand::Merge {
            amount,
            source,
            destination,
            delete_after_merge,
            usage,
        } => todo!(),
        PileSubcommand::Balance { name } => todo!(),
        PileSubcommand::List => todo!(),
        PileSubcommand::View { name } => todo!(),
        PileSubcommand::Remove { name } => todo!(),
        PileSubcommand::Commit { message } => todo!(),
        PileSubcommand::Revert { message } => todo!(),
        PileSubcommand::Revert { message } => todo!(),
        PileSubcommand::Restore => todo!(),
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
