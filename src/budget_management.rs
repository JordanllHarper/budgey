use std::fs;

use crate::{
    models::budget::Budget,
    utils::{concat_paths, create_json_file_name, create_json_path},
    BudgeyContext,
};

pub fn create_new_budget(budget_path: &str, budget: Budget) -> anyhow::Result<()> {
    fs::create_dir(&budget_path)?;
    let budget_file_path = create_json_path(budget_path, &budget.budget_detail.budget_name);
    fs::write(&budget_file_path, serde_json::to_string(&budget)?)?;
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
    let budget_path = concat_paths(&budgey_context.budgey_path, &budget_name);
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
