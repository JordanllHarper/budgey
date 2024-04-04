use clap::Parser;
use models::budgey_state::BudgeyState;
use utils::{concat_path_and_name, create_json_path};

pub mod budget_management;
pub mod budgey_cli;
mod budgey_state;
pub mod models;
pub mod utils;

struct BudgeyContext {
    budgey_path: String,
    state: BudgeyState,
}

impl BudgeyContext {
    fn new(budgey_path: String, state: BudgeyState) -> Self {
        Self { budgey_path, state }
    }
}

fn main() {
    let home = env!("HOME").to_string();
    let budgey_path = format!("{}{}", home, "/budgey/");
    let budgey_state_json_name = "budgey_state";
    let budgey_state_path = create_json_path(&budgey_path, budgey_state_json_name);
    let state = get_or_create_budgey_state(&budgey_state_path, budgey_state_json_name).unwrap();
    println!("{:?}", state);
}
fn get_or_create_budgey_state(
    budgey_state_path: &str,
    json_name: &str,
) -> anyhow::Result<BudgeyState> {
    // Steps:
    // 1. Check if state exists
    // 2. If it does, return it
    // 3. If it doesn't, create a new state and return it

    let budgey_state_result = budgey_state::get_budgey_state(budgey_state_path);
    match budgey_state_result {
        Ok(state) => Ok(state),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // state not found, create new one
                let new_state = BudgeyState::new_init();
                budgey_state::create_budgey_state(budgey_state_path, json_name, &new_state)?;
                Ok(new_state)
            } else {
                Err(e.into())
            }
        }
    }
}
