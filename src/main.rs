use anyhow::Ok;
use clap::Parser;
use log::{info, trace};
use models::budgey_state::BudgeyState;
use utils::{concat_paths, create_json_file_name};

mod budget_management;
mod budgey_cli;
mod budgey_state;
mod handle_budget;
mod handle_init;
mod handle_pile;
mod models;
mod pile_management;
mod utils;

#[derive(Debug, Clone)]
pub struct BudgeyContext {
    budgey_config: BudgeyConfig,
    state: BudgeyState,
}

#[derive(Debug, Clone)]
pub struct BudgeyConfig {
    budgey_path: String,
    state_json_name: String,
}

impl BudgeyConfig {
    pub fn new(budgey_path: &str, state_json_name: &str) -> Self {
        Self {
            budgey_path: budgey_path.to_string(),
            state_json_name: state_json_name.to_string(),
        }
    }
    pub fn get_state_path(&self) -> String {
        concat_paths(&self.budgey_path, &self.state_json_name)
    }
    pub fn get_budget_path(&self, budget_name: &str) -> String {
        concat_paths(&self.budgey_path, budget_name)
    }
}

impl BudgeyContext {
    pub fn new(state: &BudgeyState, budgey_config: &BudgeyConfig) -> Self {
        Self {
            budgey_config: budgey_config.clone(),
            state: state.clone(),
        }
    }
    pub fn get_current_budget_path(&self) -> String {
        let current_budget = &self.state.current_focused_budget_name;
        let budgey_path = &self.budgey_config.budgey_path;
        concat_paths(&budgey_path, &current_budget)
    }
    pub fn get_current_budget_json_path(&self) -> String {
        concat_paths(
            &self.get_current_budget_path(),
            &create_json_file_name(&self.state.current_focused_budget_name),
        )
    }
}

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let home = env!("HOME").to_string();
    info!("Home environment initialised: {}", home);
    let budgey_path = format!("{}{}", home, "/budgey");
    info!("Budgey path: {}", budgey_path);
    let budgey_state_json_name = create_json_file_name("budgey_state");
    let budgey_state_path = concat_paths(&budgey_path, &budgey_state_json_name);
    info!("Budgey state path created: {}", budgey_state_path);
    let budgey_config = BudgeyConfig::new(&budgey_path, &budgey_state_json_name);
    info!("Budgey config created:\n {:#?}", budgey_config);
    info!("Parsing CLI arguments");

    let args = budgey_cli::BudgeyCLI::parse();
    trace!("Parsed CLI arguments: {:#?}", args);

    let _ = match args.commands {
        budgey_cli::Commands::Init { name } => handle_init::handle_init(budgey_config, &name),
        budgey_cli::Commands::Budget { subcommand } => {
            let budgey_state = budgey_state::get_budgey_state(&budgey_state_path)?;
            let context = BudgeyContext::new(&budgey_state, &budgey_config);
            if let Some(command) = subcommand {
                handle_budget::handle_budget_subcommand(&context, command)?;
            } else {
                let current_budget = budget_management::get_current_budget_name(&context)?;
                println!("Current budget: {:?}", current_budget);
            }
            Ok(())
        }
        budgey_cli::Commands::Pile { subcommand } => {
            let budgey_state = budgey_state::get_budgey_state(&budgey_state_path)?;
            let context = BudgeyContext::new(&budgey_state, &budgey_config);
            if let Some(p) = subcommand {
                handle_pile::handle_pile_subcommand(context, p)?;
            } else {
                let current_pile = pile_management::get_current_pile(&context)?;
                println!("Current pile: {}", current_pile.get_name());
            }
            Ok(())
        }
    };
    Ok(())
}
