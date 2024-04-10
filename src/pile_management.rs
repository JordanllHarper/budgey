use std::fs;

use crate::{
    budget_management::get_current_budget,
    models::pile::Pile,
    utils::{concat_paths, create_json_path},
    BudgeyContext,
};
pub fn get_current_pile(context: &BudgeyContext) -> anyhow::Result<Pile> {
    let current_budget = get_current_budget(context)?;
    get_pile(context, &current_budget.current_pile_name)
}
pub fn get_pile(context: &BudgeyContext, pile_name: &str) -> anyhow::Result<Pile> {
    let pile_path = concat_paths(&context.get_current_budget_path(), &pile_name);
    let pile_json_path = create_json_path(&pile_path, &pile_name);
    let pile_json = fs::read_to_string(pile_json_path)?;
    Ok(serde_json::from_str::<Pile>(&pile_json)?)
}

pub fn maybe_get_pile(context: &BudgeyContext, pile_name: &str) -> anyhow::Result<Option<Pile>> {
    let current_budget = get_current_budget(context)?;
    let in_budget_ledger = current_budget.pile_names.contains(&pile_name.into());
    if !in_budget_ledger {
        return Ok(None);
    }

    let pile = get_pile(context, pile_name)?;
    Ok(Some(pile))
}

pub fn create_new_pile(context: &BudgeyContext, pile: &Pile) -> anyhow::Result<()> {
    let current_budget_path = context.get_current_budget_path();
    let pile_name = pile.get_name();
    let pile_directory_path = concat_paths(&current_budget_path, &pile_name);
    fs::create_dir(&pile_directory_path)?;
    let pile_file_path =
        create_json_path(&concat_paths(&current_budget_path, &pile_name), &pile_name);
    fs::write(&pile_file_path, serde_json::to_string(&pile)?)?;
    Ok(())
}
pub fn delete_pile(context: &BudgeyContext, pile_name: &str) -> anyhow::Result<()> {
    let pile_path = concat_paths(&context.get_current_budget_path(), &pile_name);
    fs::remove_dir_all(pile_path)?;
    Ok(())
}
