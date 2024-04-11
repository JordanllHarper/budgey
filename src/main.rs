use anyhow::Ok;
use budget_management::update_budget;
use budgey_state::write_budgey_state;
use clap::Parser;
use log::{info, trace};
use models::budgey_state::BudgeyState;
use utils::{concat_paths, create_json_file_name};

use crate::{
    budget_management::{create_new_budget, get_current_budget},
    models::{budget::Budget, pile::Pile},
    pile_management::{create_new_pile, get_current_pile, maybe_get_pile},
};

mod budget_management;
mod budgey_cli;
mod budgey_state;
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
        budgey_cli::Commands::Init { name } => handle_init(budgey_config, &name),
        budgey_cli::Commands::Budget { subcommand } => {
            let budgey_state = budgey_state::get_budgey_state(&budgey_state_path)?;
            let context = BudgeyContext::new(&budgey_state, &budgey_config);
            if let Some(command) = subcommand {
                handle_budget_subcommand(&context, command)?;
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
                handle_pile(context, p)?;
            } else {
                let current_pile = pile_management::get_current_pile(&context)?;
                println!("Current pile: {}", current_pile.get_name());
            }
            Ok(())
        }
    };
    Ok(())
}

fn handle_init(budgey_config: BudgeyConfig, starting_budget_name: &str) -> anyhow::Result<()> {
    let check_budgey_state_initialised =
        budgey_state::check_budgey_state_initialised(&budgey_config)?;

    if check_budgey_state_initialised {
        println!("Budgey already initialised. Creating budget instead. ");
    } else {
        println!("Initialising Budgey...");
        budgey_state::write_budgey_state(
            &budgey_config,
            &BudgeyState::new_init(starting_budget_name),
        )?;
    }
    let budget_path = concat_paths(&budgey_config.budgey_path, starting_budget_name);
    println!("Creating new budget: {}", starting_budget_name);
    create_new_budget(&budget_path, Budget::new_init(starting_budget_name))?;

    let context = BudgeyContext::new(&BudgeyState::new_init(starting_budget_name), &budgey_config);
    create_new_pile(&context, &Pile::default_main_pile())?;

    println!("Budgey init finished. Run `budgey` to see help.");
    Ok(())
}

fn handle_budget_subcommand(
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
            write_budgey_state(&context.budgey_config, &new_state)?;
            println!("Checked out new budget: {}", name);
            Ok(())
        }
        budgey_cli::BudgetSubcommand::New { name } => {
            let budget_exists = budget_management::does_budget_exist(&context, &name)?;
            if budget_exists {
                println!("Budget already exists with the same name");
                return Ok(());
            }
            create_new_budget(
                &context.budgey_config.get_budget_path(&name),
                Budget::new_init(&name),
            )?;
            let new_state = context
                .state
                .add_budget_name(&name)
                .change_focused_budget_name(&name);
            write_budgey_state(&context.budgey_config, &new_state)?;
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
        budgey_cli::BudgetSubcommand::List => {
            let budget_names = &context.state.budget_names;

            if budget_names.is_empty() {
                println!("No budgets found. Type budgey budget new <name> to create a new budget.");
                return Ok(());
            };
            println!("Budgets: ");

            for name in budget_names {
                if name == &context.state.current_focused_budget_name {
                    println!(" * {}", name);
                } else {
                    println!(" - {}", name);
                }
            }

            Ok(())
        }
    }
}

fn handle_pile(
    context: BudgeyContext,
    subcommand: budgey_cli::PileSubcommand,
) -> anyhow::Result<()> {
    trace!("Handling pile subcommand: {:?}", subcommand);
    match subcommand {
        budgey_cli::PileSubcommand::New {
            source,
            new_pile_name,
            initial_balance,
        } => {
            let initial_balance = initial_balance.unwrap_or(0.0);
            let pile = match maybe_get_user_defined_pile(&context, source.as_deref())? {
                Some(source_pile) => source_pile,
                None => {
                    println!("Couldn't get the source pile specified");
                    return Ok(());
                }
            };

            let new_pile = Pile::new_user_created(initial_balance, &new_pile_name, &pile.records);
            create_new_pile(&context, &new_pile)?;
            let budget = get_current_budget(&context)?
                .add_pile(&new_pile_name)
                .change_pile_name(&new_pile_name);
            update_budget(&context.get_current_budget_path(), budget)?;

            Ok(())
        }
        budgey_cli::PileSubcommand::List => {
            let current_budget = get_current_budget(&context)?;
            let pile_names = &current_budget.pile_names;

            if pile_names.is_empty() {
                println!("No piles found. Type budgey pile new <name> to create a new pile.");
                return Ok(());
            };
            println!("Piles: ");

            for name in pile_names {
                if name == &current_budget.current_pile_name {
                    println!(" * {}", name);
                } else {
                    println!(" | {}", name);
                }
            }

            Ok(())
        }
        budgey_cli::PileSubcommand::Delete { name } => {
            pile_management::delete_pile(&context, &name)?;
            let current_budget = get_current_budget(&context)?.delete_pile(&name);
            update_budget(&context.get_current_budget_path(), current_budget)
        }
    }
}
fn maybe_get_user_defined_pile(
    context: &BudgeyContext,
    maybe_pile_name: Option<&str>,
) -> anyhow::Result<Option<Pile>> {
    if let Some(p) = maybe_pile_name {
        maybe_get_pile(context, p)
    } else {
        Ok(Some(get_current_pile(context)?))
    }
}
