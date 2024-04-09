use std::fs;

use crate::{
    models::pile::Pile,
    utils::{concat_paths, create_json_path},
};

pub fn create_new_pile(budget_path_root: &str, pile: Pile) -> anyhow::Result<()> {
    let pile_name = pile.get_name();
    let pile_directory_path = concat_paths(budget_path_root, &pile_name);
    fs::create_dir(&pile_directory_path)?;
    let pile_file_path = create_json_path(&concat_paths(&budget_path_root, &pile_name), &pile_name);
    fs::write(&pile_file_path, serde_json::to_string(&pile)?)?;
    Ok(())
}
