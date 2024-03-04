use std::fs::{self, ReadDir};

use crate::models::pile::Pile;
pub enum CreateNewPileError {
    BudgeyDirectoryDoesntExist,
    // Clarification - This means that either none exist or more than one exists
    OneNamedBudgetDirDoesntExist,
    BudgetCouldntBeRead,
    CouldntCreatePileDir,
    ConvertingPileToJsonFailed,
    WritingJsonFailed,
}

pub trait PileRepository {
    fn create_new_pile(
        &self,
        pile: Pile,
        budget_name: &str,
    ) -> anyhow::Result<(), CreateNewPileError>;
    fn get_all_piles(&self) -> anyhow::Result<(), CreateNewPileError>;
    fn update_pile(&self, pile: Pile) -> anyhow::Result<(), CreateNewPileError>;
    // TODO: Remember to implement logic to not allow deleting of the main pile.
    fn delete_pile(&self, pile_name: &str) -> anyhow::Result<(), CreateNewPileError>;
}
pub struct PileRepositoryImpl {
    pub budgey_path: String,
}

impl PileRepositoryImpl {
    pub fn new(budgey_path: &str) -> Self {
        Self {
            budgey_path: budgey_path.to_string(),
        }
    }
}

impl PileRepository for PileRepositoryImpl {
    fn create_new_pile(
        &self,
        pile: Pile,
        budget_name: &str,
    ) -> anyhow::Result<(), CreateNewPileError> {
        todo!()
    }

    fn get_all_piles(&self) -> anyhow::Result<(), CreateNewPileError> {
        todo!()
    }

    fn update_pile(&self, pile: Pile) -> anyhow::Result<(), CreateNewPileError> {
        todo!()
    }

    fn delete_pile(&self, pile_name: &str) -> anyhow::Result<(), CreateNewPileError> {
        todo!()
    }
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
