use std::fs;

use crate::{
    budgey_state::write_budgey_state,
    models::budget::Budget,
    utils::{concat_paths, create_json_path},
    BudgeyContext,
};

pub fn update_budget(budget_path: &str, budget: &Budget) -> anyhow::Result<()> {
    let budget_file_path = create_json_path(budget_path, &budget.budget_detail.budget_name);
    fs::write(budget_file_path, serde_json::to_string(&budget)?)?;
    Ok(())
}
pub fn delete_budget(budgey_context: &BudgeyContext, budget_name: &str) -> anyhow::Result<()> {
    let budget_path = concat_paths(&budgey_context.config.root_path, budget_name);
    fs::remove_dir_all(budget_path)?;
    let new_state = budgey_context.state.remove_budget_name(budget_name);
    write_budgey_state(&budgey_context.config, &new_state)?;
    Ok(())
}
