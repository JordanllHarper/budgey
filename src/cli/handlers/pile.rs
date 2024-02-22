use crate::cli::budgeter_cli::PileSubcommand;

pub trait PileHandler {
    fn handle(&self, subcommand: PileSubcommand) -> anyhow::Result<()>;
}
impl dyn PileHandler {
    fn handle(&self, subcommand: PileSubcommand) -> anyhow::Result<()> {
        match subcommand {
            PileSubcommand::New {
                name,
                initial_balance,
            } => todo!(),
            PileSubcommand::Push { amount, source } => todo!(),
            PileSubcommand::Pull { amount, usage } => todo!(),
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
        }
        todo!()
    }
}
