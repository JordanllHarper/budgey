use std::fs;

use log::trace;

use crate::{models::budgey_state::BudgeyState, utils::concat_paths, BudgeyConfig};

/// Gets the budgey state from the given path
pub fn get_budgey_state(budgey_path_to_json: &str) -> anyhow::Result<BudgeyState, std::io::Error> {
    trace!("Getting budgey state");
    let read_result = fs::read_to_string(budgey_path_to_json)?;
    let state: BudgeyState = serde_json::from_str(&read_result)?;
    Ok(state)
}

/// Writes a new budgey state file with the given path and state
pub fn write_budgey_state(
    budgey_config: &BudgeyConfig,
    new_state: &BudgeyState,
) -> anyhow::Result<(), std::io::Error> {
    trace!("Writing budgey state");
    let serialized = serde_json::to_string(new_state)?;
    let check_path_result = fs::read_dir(&budgey_config.budgey_path);
    match check_path_result {
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                fs::create_dir_all(&budgey_config.budgey_path)?;
            }
        }
        _ => {}
    };
    fs::write(
        concat_paths(&budgey_config.budgey_path, &budgey_config.state_json_name),
        serialized,
    )?;

    Ok(())
}
