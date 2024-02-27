use clap::Parser;
use cli::{
    budgeter_cli,
    handlers::init::{InitUseCase, InitUseCaseError, InitUseCaseImpl},
};
use repo::budget_repository::BudgetRepositoryImpl;
use utils::error::HandlerError;

use crate::config::local_config::LocalConfig;

pub mod cli;
pub mod config;
pub mod models;
pub mod repo;
pub mod utils;

fn main() {
    let local_config = LocalConfig::test();
    let init_use_case: InitUseCaseImpl<BudgetRepositoryImpl> = InitUseCaseImpl::new(
        &local_config.budgey_dir,
        BudgetRepositoryImpl::new(&local_config.budgey_dir),
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
fn print_error(error: &impl HandlerError) {
    println!("{}", error.get_user_message());
}
