use crate::cli::budgeter_cli::{PileOperationSubcommand, PileSubcommand};

pub trait PileHandler {
    fn handle(&self, subcommand: PileSubcommand) -> anyhow::Result<()>;
}
// TODO: create handlers and delegate
impl dyn PileHandler {
    fn handle(&self, subcommand: PileSubcommand) -> anyhow::Result<()> {
        match subcommand {
            PileSubcommand::New {
                name,
                initial_balance,
            } => todo!(),
            PileSubcommand::PileOperation(subcommand) => {
                Self::handle_pile_op_subcommand(subcommand)
            }
        }
    }

    fn handle_pile_op_subcommand(subcommand: PileOperationSubcommand) -> anyhow::Result<()> {
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
        todo!()
    }
}
