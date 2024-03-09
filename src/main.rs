use clap::Parser;
use cli::{budgeter_cli, use_cases::init::handle_init};

use crate::config::local_config::LocalConfig;

pub mod cli;
pub mod config;
pub mod handling;
pub mod io_operations;
pub mod models;

fn main() {
    let local_config = LocalConfig::default();
    let budgey_path = &local_config.budgey_dir;
    // commands
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
    let result = match commands {
        budgeter_cli::Commands::Init { name } => handle_init(&name, budgey_path),
        budgeter_cli::Commands::Pile { subcommand } => todo!(),
    };
    if let Err(e) = result {
        println!("{:?}", e)
    };
}
