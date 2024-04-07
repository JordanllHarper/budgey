use std::fs;

use crate::{models::budgey_state::BudgeyState, utils::concat_paths};

/// Gets the budgey state from the given path
pub fn get_budgey_state(budgey_path_to_json: &str) -> anyhow::Result<BudgeyState, std::io::Error> {
    let read_result = fs::read_to_string(budgey_path_to_json)?;
    let state: BudgeyState = serde_json::from_str(&read_result)?;
    Ok(state)
}

/// Creates a new budgey state file with the given path and state
pub fn create_budgey_state(
    budgey_path: &str,
    json_name: &str,
    new_state: &BudgeyState,
) -> anyhow::Result<(), std::io::Error> {
    let serialized = serde_json::to_string(new_state)?;
    let check_path_result = fs::read_dir(budgey_path);
    match check_path_result {
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                fs::create_dir_all(budgey_path)?;
            }
        }
        _ => {}
    };
    fs::write(concat_paths(budgey_path, json_name), serialized)?;

    Ok(())
}
