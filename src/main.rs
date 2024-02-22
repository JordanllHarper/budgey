use clap::Parser;
use cli::{budgeter_cli, handlers::init::InitHandler};

use crate::cli::handlers::init::InitHandlerImpl;

pub mod cli;
pub mod config;
pub mod models;
pub mod repo;
fn main() {
    let init_handler: InitHandlerImpl = InitHandlerImpl::new();
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
    match commands {
        budgeter_cli::Commands::Init { name } => handle_init(&init_handler, &name),
        budgeter_cli::Commands::Pile { subcommand } => todo!(),
    };
    todo!()
}
fn handle_init(handler: &impl InitHandler, repo_name: &str) -> anyhow::Result<()> {
    handler.handle(repo_name)
}
