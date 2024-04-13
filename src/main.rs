use crate::budgey_state::BudgeyState;
use anyhow::Ok;
use clap::Parser;
use log::{info, trace};
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
        concat_paths(budgey_path, current_budget)
    }
    pub fn get_current_budget_json_path(&self) -> String {
        concat_paths(
            &self.get_current_budget_path(),
            &create_json_file_name(&self.state.current_focused_budget_name),
        )
    }

    pub fn update_state(&self, new_state: &BudgeyState) -> Self {
        Self::new(new_state, &self.budgey_config)
    }
}
fn is_budgey_initialised(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

fn main() -> anyhow::Result<()> {
    simple_logger::init().unwrap();

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
            handle_non_init(&budgey_config, &budgey_state_path, |context| {
                if let Some(sub) = &subcommand {
                    handle_budget::handle_budget_subcommand(&context, sub.clone())
                } else {
                    let current_budget = budget_management::get_current_budget_name(&context)?;
                    println!("Current budget: {:?}", current_budget);
                    Ok(())
                }
            })
        }
        budgey_cli::Commands::Pile { subcommand } => {
            handle_non_init(&budgey_config, &budgey_state_path, |context| {
                if let Some(sub) = &subcommand {
                    return handle_pile::handle_pile_subcommand(context, sub.clone());
                } else {
                    let current_pile = pile_management::get_current_pile(&context)?;
                    println!("Current pile: {}", current_pile.get_name());
                }
                Ok(())
            })
        }
    };
    Ok(())
}
fn handle_non_init(
    budgey_config: &BudgeyConfig,
    budgey_state_path: &str,
    f: impl Fn(BudgeyContext) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    if !is_budgey_initialised(&budgey_config.budgey_path) {
        info!("Budgey is not initialised");
        println!(
            "Budgey is not initialised, run `budgey init <starting budget name>` to initialise"
        );
        return Ok(());
    }
    let budgey_state = budgey_state::get_budgey_state(budgey_state_path)?;
    let context = BudgeyContext::new(&budgey_state, budgey_config);

    f(context)
}
