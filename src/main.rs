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

pub struct BudgeyContext {
    budgey_path: String,
    state: BudgeyState,
}

impl BudgeyContext {
    fn new(budgey_path: &str, state: BudgeyState) -> Self {
        Self {
            budgey_path: budgey_path.to_string(),
            state,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let home = env!("HOME").to_string();
    let budgey_path = format!("{}{}", home, "/budgey/");
    let budgey_state_json_name = create_json_file_name("budgey_state");
    let budgey_state_path = concat_paths(&budgey_path, &budgey_state_json_name);

    let args = budgey_cli::BudgeyCLI::parse();

    let result = match args.commands {
        budgey_cli::Commands::Init { name } => {
            handle_init(&budgey_path, &budgey_state_json_name, &name)
        }
        budgey_cli::Commands::Budget { subcommand } => {
            let budgey_state = budgey_state::get_budgey_state(&concat_paths(
                &budgey_state_path,
                &budgey_state_json_name,
            ))?;
            let context = BudgeyContext::new(&budgey_path, budgey_state);
            handle_budget(context, subcommand)
        }
        budgey_cli::Commands::Pile { subcommand } => {
            let budgey_state = budgey_state::get_budgey_state(&concat_paths(
                &budgey_state_path,
                &budgey_state_json_name,
            ))?;
            let context = BudgeyContext::new(&budgey_path, budgey_state);
            handle_pile(context, subcommand)
        }
    };
    result
}

fn handle_init(
    budgey_path: &str,
    json_name: &str,
    starting_budget_name: &str,
) -> anyhow::Result<()> {
    budgey_state::create_budgey_state(
        budgey_path,
        json_name,
        &BudgeyState::new_init(starting_budget_name),
    )
    .unwrap();

    let budget_path = concat_paths(budgey_path, starting_budget_name);
    create_new_budget(&budget_path, Budget::new(starting_budget_name)).unwrap();
    create_new_pile(&budget_path, Pile::default_main_pile())?;
    println!("Budgey initialised");
    Ok(())
}

fn handle_budget(
    context: BudgeyContext,
    subcommand: budgey_cli::BudgetSubcommand,
) -> anyhow::Result<()> {
    todo!()
}

fn handle_pile(
    context: BudgeyContext,
    subcommand: budgey_cli::PileSubcommand,
) -> anyhow::Result<()> {
    todo!()
}
// fn get_or_create_budgey_state(
//     budgey_state_path: &str,
//     json_name: &str,
// ) -> anyhow::Result<BudgeyState> {
//     let budgey_to_json = concat_path_and_name(budgey_state_path, json_name);
//     let budgey_state_result = budgey_state::get_budgey_state(&budgey_to_json);
//     match budgey_state_result {
//         Ok(state) => Ok(state),
//         Err(e) => {
// if e.kind() == std::io::ErrorKind::NotFound {
// let new_state = BudgeyState::new_init();
// budgey_state::create_budgey_state(budgey_state_path, json_name, &new_state)
//     .unwrap();
// Ok(new_state)
// } else {
//     println!("{:?}", e);
//     Err(e.into())
// }
//         }
//     }
// }
