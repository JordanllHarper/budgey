use clap::Parser;
use cli::{budgeter_cli, use_cases::init::handle_init};

use crate::{cli::use_cases::init::InitBudgetData, config::local_config::LocalConfig};

pub mod cli;
pub mod config;
pub mod handling;
pub mod io_operations;
pub mod models;

fn main() {
    let local_config = LocalConfig::default();
    let budgey_path = &local_config.budgey_dir;
    println!("Budgey path: {}", budgey_path);
    // commands
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
    let result = match commands {
        budgeter_cli::Commands::Init { name } => handle_init(
            InitBudgetData::new(budgey_path.to_string(), name),
            handling::budgey_handling::init_budgey,
            handling::budget_handling::create_new_budget,
            handling::pile_handling::create_new_pile,
        ),
        budgeter_cli::Commands::Pile { subcommand } => todo!(),
    };
    if let Err(e) = result {
        println!("{}", e)
    };
}
