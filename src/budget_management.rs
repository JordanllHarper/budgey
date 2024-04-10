use std::fs;

use crate::{
    budgey_state::write_budgey_state,
    models::budget::Budget,
    utils::{concat_paths, create_json_file_name, create_json_path},
    BudgeyContext,
};
pub fn get_current_budget_name(budgey_context: &BudgeyContext) -> anyhow::Result<String> {
    Ok(budgey_context.state.current_focused_budget_name.to_string())
}

pub fn get_current_budget(budgey_context: &BudgeyContext) -> anyhow::Result<Budget> {
    let current_budget_path = &budgey_context.get_current_budget_json_path();
    let current_budget_json = fs::read_to_string(&current_budget_path)?;
    let current_budget: Budget = serde_json::from_str(&current_budget_json)?;
    Ok(current_budget)
}
pub fn create_new_budget(budget_path: &str, budget: Budget) -> anyhow::Result<()> {
    fs::create_dir(&budget_path)?;
    let budget_file_path = create_json_path(budget_path, &budget.budget_detail.budget_name);
    fs::write(&budget_file_path, serde_json::to_string(&budget)?)?;
    Ok(())
}
pub fn update_budget(budget_path: &str, budget: Budget) -> anyhow::Result<()> {
    let budget_file_path = create_json_path(budget_path, &budget.budget_detail.budget_name);
    fs::write(&budget_file_path, serde_json::to_string(&budget)?)?;
    Ok(())
}
pub fn delete_budget(budgey_context: &BudgeyContext, budget_name: &str) -> anyhow::Result<()> {
    let budget_path = concat_paths(&budgey_context.budgey_config.budgey_path, &budget_name);
    fs::remove_dir_all(budget_path)?;

    let new_state = budgey_context.state.remove_budget_name(&budget_name);
    write_budgey_state(&budgey_context.budgey_config, &new_state)?;
    Ok(())
}

pub fn does_budget_exist(
    budgey_context: &BudgeyContext,
    budget_name: &str,
) -> anyhow::Result<bool> {
    if budgey_context
        .state
        .budget_names
        .iter()
        .any(|name| name == budget_name)
    {
        return Ok(true);
    }
    let budget_path = concat_paths(&budgey_context.budgey_config.budgey_path, &budget_name);
    let budget_json_path = concat_paths(&budget_path, &create_json_file_name(&budget_name));

    match fs::read(budget_json_path) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(e.into())
            }
        }
    }
}
