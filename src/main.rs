use anyhow::Ok;
use budgey_state::write_budgey_state;
use clap::Parser;
use models::budgey_state::BudgeyState;
use utils::{concat_paths, create_json_file_name};

use crate::{
    budget_management::create_new_budget,
    models::{budget::Budget, pile::Pile},
    pile_management::create_new_pile,
};

pub mod budget_management;
pub mod budgey_cli;
mod budgey_state;
pub mod models;
pub mod pile_management;
pub mod utils;
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
    fn new(state: &BudgeyState, budgey_config: &BudgeyConfig) -> Self {
        Self {
            budgey_config: budgey_config.clone(),
            state: state.clone(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let home = env!("HOME").to_string();
    let budgey_path = format!("{}{}", home, "/budgey");
    let budgey_state_json_name = create_json_file_name("budgey_state");
    let budgey_state_path = concat_paths(&budgey_path, &budgey_state_json_name);
    let budgey_config = BudgeyConfig::new(&budgey_path, &budgey_state_json_name);

    let args = budgey_cli::BudgeyCLI::parse();

    let result = match args.commands {
        budgey_cli::Commands::Init { name } => handle_init(budgey_config, &name),
        budgey_cli::Commands::Budget { subcommand } => {
            let budgey_state = budgey_state::get_budgey_state(&budgey_state_path)?;
            let context = BudgeyContext::new(&budgey_state, &budgey_config);
            if let Some(command) = subcommand {
                handle_budget_subcommand(&budgey_config, &context, command)?;
            } else {
                let current_budget = budget_management::get_current_budget(&context)?;
                println!("Current budget: {}", current_budget);
            }
            Ok(())
        }
        budgey_cli::Commands::Pile { subcommand } => {
            let budgey_state = budgey_state::get_budgey_state(&concat_paths(
                &budgey_state_path,
                &budgey_state_json_name,
            ))?;
            let context = BudgeyContext::new(&budgey_state, &budgey_config);
            handle_pile(context, subcommand)
        }
    };
    result
}

fn handle_init(budgey_config: BudgeyConfig, starting_budget_name: &str) -> anyhow::Result<()> {
    budgey_state::write_budgey_state(&budgey_config, &BudgeyState::new_init(starting_budget_name))?;

    let budget_path = concat_paths(&budgey_config.budgey_path, starting_budget_name);
    create_new_budget(&budget_path, Budget::new(starting_budget_name))?;
    create_new_pile(&budget_path, Pile::default_main_pile())?;
    println!("Budgey initialised");
    Ok(())
}

fn handle_budget_subcommand(
    budgey_config: &BudgeyConfig,
    context: &BudgeyContext,
    subcommand: budgey_cli::BudgetSubcommand,
) -> anyhow::Result<()> {
    match subcommand {
        budgey_cli::BudgetSubcommand::Focus { name } => {
            let state = &context.state;
            let budget_exists = budget_management::does_budget_exist(&context, &name)?;

            if !budget_exists {
                println!("Budget doesn't exist, specify another name");
                return Ok(());
            }

            let new_state = state.change_focused_budget_name(&name);
            write_budgey_state(&budgey_config, &new_state)?;
            println!("Checked out new budget: {}", name);
            Ok(())
        }
        budgey_cli::BudgetSubcommand::New { name } => {
            let budget_exists = budget_management::does_budget_exist(&context, &name)?;
            if budget_exists {
                println!("Budget already exists with the same name");
                return Ok(());
            }
            create_new_budget(&budgey_config.get_budget_path(&name), Budget::new(&name))?;
            let new_state = context
                .state
                .add_budget_name(&name)
                .change_focused_budget_name(&name);
            write_budgey_state(&budgey_config, &new_state)?;
            println!("Created and focused new budget: {}", name);
            Ok(())
        }
        budgey_cli::BudgetSubcommand::Delete { name } => {
            let budget_exists = budget_management::does_budget_exist(&context, &name)?;
            if !budget_exists {
                println!("Budget doesn't exist, specify another name");
                return Ok(());
            }

            budget_management::delete_budget(context, &name)?;
            println!("Deleted budget: {}", name);
            Ok(())
        }
    }
}

fn handle_pile(
    context: BudgeyContext,
    subcommand: budgey_cli::PileSubcommand,
) -> anyhow::Result<()> {
    todo!()
}
