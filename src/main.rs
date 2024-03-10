use clap::Parser;
use cli::{
    budgeter_cli,
    use_cases::{
        init::{handle_init, InitError},
        pile::{handle_pile_command, PileError},
    },
};

use crate::config::local_config::LocalConfig;

pub mod cli;
pub mod config;
pub mod handling;
pub mod io_operations;
pub mod models;

#[derive(Debug)]
pub enum CommonError {
    InitError(InitError),
    PileError(PileError),
}

impl CommonError {
    pub fn wrap_init(e: InitError) -> Self {
        CommonError::InitError(e)
    }

    pub fn wrap_pile(e: PileError) -> Self {
        CommonError::PileError(e)
    }
}
impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::InitError(e) => write!(f, "{}", e),
            CommonError::PileError(e) => write!(f, "{}", e),
        }
    }
}

fn main() {
    let local_config = LocalConfig::default();
    let budgey_path = &local_config.budgey_dir;
    // commands
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
    let result = match commands {
        budgeter_cli::Commands::Init { name: budget_name } => {
            let result = handle_init(
                budgey_path,
                &budget_name,
                handling::budgey_handling::init_budgey,
                handling::budget_handling::create_new_budget,
                handling::pile_handling::create_new_pile,
            );
            result.map_err(|e| CommonError::wrap_init(e))
        }
        budgeter_cli::Commands::Pile {
            subcommand,
            budget_name,
        } => {
            let result = handle_pile_command(budgey_path, &budget_name, subcommand);
            result.map_err(|e| CommonError::wrap_pile(e))
        }
    };
    if let Err(e) = result {
        println!("{}", e)
    };
}
