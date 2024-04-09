use std::fs;

use crate::{
    models::pile::Pile,
    utils::{concat_paths, create_json_path},
    BudgeyContext,
};

pub fn create_new_pile(context: &BudgeyContext, pile: Pile) -> anyhow::Result<()> {
    let current_budget_name = context.state.current_focused_budget_name.to_string();
    let current_budget_path = context.budgey_config.get_budget_path(&current_budget_name);
    let pile_name = pile.get_name();
    let pile_directory_path = concat_paths(&current_budget_path, &pile_name);
    fs::create_dir(&pile_directory_path)?;
    let pile_file_path =
        create_json_path(&concat_paths(&current_budget_name, &pile_name), &pile_name);
    fs::write(&pile_file_path, serde_json::to_string(&pile)?)?;
    Ok(())
}
