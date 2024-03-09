use std::fs::{self, ReadDir};

use crate::{io_operations::io_operations::io_operations::create_json_path, models::pile::Pile};

#[derive(Debug)]
pub enum CreateNewPileError {
    PileDirectoryAlreadyExists,
    CouldntCreatePileDirectory,
    CouldntWriteJson,
}

pub fn create_new_pile(
    pile: Pile,
    budget_name: &str,
    budgey_path: &str,
) -> anyhow::Result<(), CreateNewPileError> {
    let pile_name = &pile.name;
    let pile_path = format!("{}/{}/{}", budgey_path, budget_name, pile_name);
    fs::create_dir(&pile_path).map_err(|e| {
        if let std::io::ErrorKind::AlreadyExists = e.kind() {
            CreateNewPileError::PileDirectoryAlreadyExists
        } else {
            CreateNewPileError::CouldntCreatePileDirectory
        }
    })?;
    let json_file = create_json_path(&pile_path, pile_name);
    fs::write(json_file, serde_json::to_string(&pile).unwrap())
        .map_err(|_| CreateNewPileError::CouldntWriteJson)?;

    Ok(())
}

fn get_all_piles(budget_name: &str) -> anyhow::Result<(), CreateNewPileError> {
    todo!()
}

fn update_pile(pile: Pile) -> anyhow::Result<(), CreateNewPileError> {
    todo!()
}

fn delete_pile(pile_name: &str) -> anyhow::Result<(), CreateNewPileError> {
    todo!()
}

fn one_budget_exists(read_dir: ReadDir, budget_name: &str) -> bool {
    read_dir
        .filter(|each| match each {
            Ok(v) => v.file_name() == budget_name,
            Err(_) => false,
        })
        .count()
        == 1
}
