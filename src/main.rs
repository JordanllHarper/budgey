use clap::Parser;
use cli::{
    budgeter_cli,
    use_cases::init::{InitUseCase, InitUseCaseImpl},
};
use repo::{budget_repository::BudgetRepositoryImpl, pile_repository::PileRepositoryImpl};
use utils::error::GenericError;

use crate::config::local_config::LocalConfig;

pub mod cli;
pub mod config;
pub mod io_operations;
pub mod models;
pub mod repo;
pub mod utils;

fn main() {
    let local_config = LocalConfig::test();
    let budgey_path = &local_config.budgey_dir;
    let init_use_case: InitUseCaseImpl<BudgetRepositoryImpl, PileRepositoryImpl> =
        InitUseCaseImpl::new(
            BudgetRepositoryImpl::new(budgey_path),
            PileRepositoryImpl::new(budgey_path),
        );
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
    let result = match commands {
        budgeter_cli::Commands::Init { name } => init_use_case.handle(&name),
        budgeter_cli::Commands::Pile { subcommand } => todo!(),
    };
    if let Err(e) = result {
        print_error(&e);
    };
}
fn print_error(error: &impl GenericError) {
    println!("{}", error.get_user_message());
}
