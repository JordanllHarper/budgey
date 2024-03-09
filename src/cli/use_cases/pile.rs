use crate::cli::budgeter_cli::{PileOperationSubcommand, PileSubcommand};

fn handle_pile_command(budget: &str, subcommand: PileSubcommand) -> anyhow::Result<()> {
    match subcommand {
        PileSubcommand::New {
            name,
            initial_balance,
            budget,
        } => todo!(),
        PileSubcommand::PileOperation { budget, subcommand } => {
            handle_pile_op_subcommand(subcommand)
        }
    }
}

pub fn handle_pile_op_subcommand(subcommand: PileOperationSubcommand) -> anyhow::Result<()> {
    match subcommand {
        PileOperationSubcommand::Push { amount, source } => todo!(),
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
    }
}
