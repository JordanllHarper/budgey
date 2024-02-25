use clap::Parser;
use cli::{
    budgeter_cli,
    handlers::init::{InitHandler, InitHandlerError},
};
use utils::error::HandlerError;

use crate::{cli::handlers::init::InitHandlerImpl, config::local_config::LocalConfig};

pub mod cli;
pub mod config;
pub mod models;
pub mod repo;
pub mod utils;

fn main() {
    let init_handler: InitHandlerImpl = InitHandlerImpl::new(LocalConfig::test());
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
