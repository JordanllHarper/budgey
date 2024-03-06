use crate::{
    cli::budgeter_cli::{PileOperationSubcommand, PileSubcommand},
    repo::{
        budget_repository::BudgetRepository, budgey_repository::BudgeyRepository,
        pile_repository::PileRepository,
    },
};

pub trait PileUseCase {
    fn handle(&self, budget: &str, subcommand: PileSubcommand) -> anyhow::Result<()>;
}
pub struct PileUseCaseImpl<'a> {
    pile_repo: &'a dyn PileRepository,
    budget_repo: &'a dyn BudgetRepository,
}

impl<'a> PileUseCaseImpl<'a> {
    pub fn new(pile_repo: &'a dyn PileRepository, budget_repo: &'a dyn BudgetRepository) -> Self {
        Self {
            pile_repo,
            budget_repo,
        }
    }
}
// TODO: create handlers and delegate
impl<'a> PileUseCase for PileUseCaseImpl<'a> {
    fn handle(&self, budget: &str, subcommand: PileSubcommand) -> anyhow::Result<()> {
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
    todo!()
}
