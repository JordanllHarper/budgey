use std::fs;

use log::{error, trace};

use crate::{
    budget_management::get_current_budget,
    models::pile::Pile,
    utils::{concat_paths, create_json_path},
    BudgeyContext,
};

pub fn get_current_pile(context: &BudgeyContext) -> anyhow::Result<Pile> {
    trace!("Getting current pile");
    let current_budget = get_current_budget(context)?;
    get_pile(context, &current_budget.current_pile_name)
}

pub fn update_pile(context: &BudgeyContext, new_pile: Pile) -> anyhow::Result<()> {
    trace!("Updating pile: {}", new_pile.get_name());
    let pile_path = concat_paths(&context.get_current_budget_path(), &new_pile.get_name());
    let pile_json_path = create_json_path(&pile_path, &new_pile.get_name());
    fs::write(pile_json_path, serde_json::to_string(&new_pile)?)?;
    Ok(())
}

pub fn get_pile(context: &BudgeyContext, pile_name: &str) -> anyhow::Result<Pile> {
    trace!("Getting pile");
    let pile_path = concat_paths(&context.get_current_budget_path(), &pile_name);
    let pile_json_path = create_json_path(&pile_path, &pile_name);
    let pile_json = fs::read_to_string(pile_json_path)?;
    Ok(serde_json::from_str::<Pile>(&pile_json)?)
}

/// Gets a pile if it exists in the current budget.
pub fn maybe_get_pile(context: &BudgeyContext, pile_name: &str) -> anyhow::Result<Option<Pile>> {
    trace!("Maybe getting pile");
    let current_budget = get_current_budget(context)?;
    let in_budget_ledger = current_budget.pile_names.contains(&pile_name.into());
    if !in_budget_ledger {
        return Ok(None);
    }

    let pile = get_pile(context, pile_name)?;
    Ok(Some(pile))
}

pub fn create_new_pile(context: &BudgeyContext, pile: &Pile) -> anyhow::Result<()> {
    trace!("Creating new pile");
    let current_budget_path = context.get_current_budget_path();
    let pile_name = pile.get_name();
    let pile_directory_path = concat_paths(&current_budget_path, &pile_name);
    match fs::create_dir(&pile_directory_path) {
        Ok(it) => it,
        Err(err) => {
            error!("Error creating pile directory: {:?}", err);
            return Err(err.into());
        }
    };
    let pile_file_path =
        create_json_path(&concat_paths(&current_budget_path, &pile_name), &pile_name);
    match fs::write(
        &pile_file_path,
        match serde_json::to_string(&pile) {
            Ok(it) => it,
            Err(e) => {
                error!("Error serializing pile: {:?}", e);
                return Err(e.into());
            }
        },
    ) {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing pile file: {:?}", e);
            return Err(e.into());
        }
    };
    Ok(())
}

pub fn delete_pile(context: &BudgeyContext, pile_name: &str) -> anyhow::Result<()> {
    trace!("Deleting pile");
    let pile_path = concat_paths(&context.get_current_budget_path(), &pile_name);
    fs::remove_dir_all(pile_path)?;
    Ok(())
}
