use std::fmt::{Display, Formatter};

use crate::{
    cli::budgeter_cli::{PileOperationSubcommand, PileSubcommand},
    handling::pile_handling::{create_new_pile, CreateNewPileError},
    models::pile::{Pile, PileType},
};

#[derive(Debug)]
pub enum PileError {
    CreateNewPileError(CreateNewPileError),
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
    subcommand: PileSubcommand,
) -> anyhow::Result<(), PileError> {
    match subcommand {
        PileSubcommand::New {
            name,
            initial_balance,
            budget_name: budget,
        } => create_new_pile(
            Pile::new(
                name,
                initial_balance.unwrap_or(0.0),
                PileType::UserCreated,
                vec![],
            ),
            &budget,
            budgey_path,
        )
        .map_err(|e| PileError::new_from_create_new_pile_error(e)),

        PileSubcommand::PileOperation {
            budget_name,
            subcommand,
        } => handle_pile_op_subcommand(subcommand, &budget_name)
            .map_err(|e| PileError::new_from_pile_operation_error(e)),
    }
}

#[derive(Debug)]
pub enum PileOperationError {
    PushError,
    PullError,
    MergeError,
    BalanceError,
    ListError,
    ViewError,
    RemoveError,
}
fn handle_pile_op_subcommand(
    subcommand: PileOperationSubcommand,
    budget: &str,
) -> anyhow::Result<(), PileOperationError> {
    match subcommand {
        PileOperationSubcommand::Add { amount, source } => todo!(),
        PileOperationSubcommand::Pull { amount, usage } => todo!(),
        PileOperationSubcommand::Merge {
            amount,
            source,
            destination,
            delete_after_merge,
            usage,
        } => todo!(),
        PileOperationSubcommand::Balance { name } => todo!(),
        PileOperationSubcommand::List => todo!(),
        PileOperationSubcommand::View { name } => todo!(),
        PileOperationSubcommand::Remove { name } => todo!(),
        PileOperationSubcommand::Commit { message } => todo!(),
        PileOperationSubcommand::Revert { message } => todo!(),
    }
}
