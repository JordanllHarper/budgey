use clap::Parser;
use cli::{
    budgeter_cli,
    handlers::init::{InitHandler, InitHandlerError},
};
use repo::budget_repository::BudgetRepositoryImpl;
use utils::error::HandlerError;

use crate::{cli::handlers::init::InitHandlerImpl, config::local_config::LocalConfig};

pub mod cli;
pub mod config;
pub mod models;
pub mod repo;
pub mod utils;

fn main() {
    let local_config = LocalConfig::test();
    let init_handler: InitHandlerImpl<BudgetRepositoryImpl> = InitHandlerImpl::new(
        &local_config.budgey_dir,
        BudgetRepositoryImpl::new(&local_config.budgey_dir),
    );
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
    let result = match commands {
        budgeter_cli::Commands::Init { name } => handle_init(&init_handler, &name),
        budgeter_cli::Commands::Pile { subcommand } => todo!(),
    };
    if let Err(e) = result {
        print_error(&e);
    };
}
fn print_error(error: &impl HandlerError) {
    println!("{}", error.get_user_message());
}
fn handle_init(
    handler: &impl InitHandler,
    repo_name: &str,
) -> anyhow::Result<(), InitHandlerError> {
    handler.handle(repo_name)
}
