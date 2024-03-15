use clap::Parser;
use cli::{
    budgeter_cli,
    use_cases::{
        init::{handle_init, InitError},
        pile::PileError,
    },
};
use handling::{
    budget_handling::create_new_budget,
    budgey_handling::{init_budgey, LocalConfig},
    pile_handling::create_new_pile,
};

pub mod cli;
pub mod config;
pub mod handling;
pub mod models;
pub mod utils;

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
    let budget_state_name = "budget_state";
    // commands
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
    let result = match commands {
        budgeter_cli::Commands::Init { name: budget_name } => {
            let result = handle_init(
                &budget_name,
                || init_budgey(budgey_path, budget_state_name),
                |budget| create_new_budget(&budgey_path, budget),
                |budget_name| {
                    create_new_pile(models::pile::Pile::default(), &budget_name, budgey_path)
                },
            );
            result.map_err(|e| CommonError::wrap_init(e))
        }
        budgeter_cli::Commands::Pile { subcommand } => {
            todo!()
        }
        budgeter_cli::Commands::Budget { subcommand } => todo!(),
    };
    if let Err(e) = result {
        println!("{}", e)
    };
}
