use std::{fs, io};

use log::{error, trace};

use crate::{
    budgey_state::write_budgey_state,
    models::budget::Budget,
    utils::{concat_paths, create_json_file_name, create_json_path},
    BudgeyContext,
};
pub fn get_current_budget_name(budgey_context: &BudgeyContext) -> anyhow::Result<String> {
    Ok(budgey_context.state.current_focused_budget_name.to_string())
}

pub fn get_current_budget(
    budgey_context: &BudgeyContext,
    read_to_string: fn(&str) -> io::Result<String>,
) -> anyhow::Result<Budget> {
    trace!("Getting current budget");
    let current_budget_path = &budgey_context.get_current_budget_json_path();
    let read_to_string = read_to_string(current_budget_path);
    let current_budget_json = match read_to_string {
        Ok(json) => json,
        Err(e) => {
            error!("Error reading current budget json: {:?}", e);
            return Err(e.into());
        }
    };
    let current_budget: Budget = match serde_json::from_str(&current_budget_json) {
        Ok(budget) => budget,
        Err(e) => {
            error!(
                "Error deserializing current budget json {:?}",
                current_budget_json
            );
            return Err(e.into());
        }
    };
    Ok(current_budget)
}
pub fn create_new_budget(budget_path: &str, budget: Budget) -> anyhow::Result<()> {
    trace!("Creating new budget");
    match fs::create_dir(budget_path) {
        Ok(_) => {}
        Err(e) => {
            if let std::io::ErrorKind::AlreadyExists = e.kind() {
                error!(
                    "Budget {} already exists at: {}",
                    budget.budget_detail.budget_name, budget_path
                );
                println!(
                    "It looks like a budget with the name {} already exists. Please choose a different name.",
                    budget.budget_detail.budget_name
                );
                return Err(e.into());
            }
            error!("Error creating budget directory at: {}", budget_path);
            return Err(e.into());
        }
    };
    let budget_file_path = create_json_path(budget_path, &budget.budget_detail.budget_name);
    match fs::write(
        budget_file_path,
        match serde_json::to_string(&budget) {
            Ok(it) => it,
            Err(e) => {
                error!("Error serializing budget: {:?}", e);
                return Err(e.into());
            }
        },
    ) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.into());
        }
    };
    Ok(())
}
pub fn update_budget(budget_path: &str, budget: Budget) -> anyhow::Result<()> {
    trace!("Updating budget");
    let budget_file_path = create_json_path(budget_path, &budget.budget_detail.budget_name);
    match fs::write(
        &budget_file_path,
        match serde_json::to_string(&budget) {
            Ok(v) => v,
            Err(e) => {
                error!("Error serializing budget: {:?}", budget);
                return Err(e.into());
            }
        },
    ) {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing budget file to {}: {:?}", budget_file_path, e);
            return Err(e.into());
        }
    };
    Ok(())
}
pub fn delete_budget(budgey_context: &BudgeyContext, budget_name: &str) -> anyhow::Result<()> {
    trace!("Deleting budget");
    let budget_path = concat_paths(&budgey_context.budgey_config.budgey_path, budget_name);
    match fs::remove_dir_all(&budget_path) {
        Ok(_) => {}
        Err(e) => {
            error!(
                "Error removing budget directory at {}: {:?}",
                budget_path, e
            );
            return Err(e.into());
        }
    };

    let new_state = budgey_context.state.remove_budget_name(budget_name);
    match write_budgey_state(&budgey_context.budgey_config, &new_state) {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing new state after deleting budget: {:?}", e);
            return Err(e.into());
        }
    };
    Ok(())
}

pub fn does_budget_exist(
    budgey_context: &BudgeyContext,
    budget_name: &str,
) -> anyhow::Result<bool> {
    trace!("Checking if budget exists");
    if budgey_context
        .state
        .budget_names
        .iter()
        .any(|name| name == budget_name)
    {
        return Ok(true);
    }
    let budget_path = concat_paths(&budgey_context.budgey_config.budgey_path, budget_name);
    let budget_json_path = concat_paths(&budget_path, &create_json_file_name(budget_name));

    match fs::read(&budget_json_path) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(false)
            } else {
                error!(
                    "Error reading budget json at path {}: {:?}",
                    budget_json_path, e
                );
                Err(e.into())
            }
        }
    }
}
