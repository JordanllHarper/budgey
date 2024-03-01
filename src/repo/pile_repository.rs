use std::fs::{self};

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
        //Check if budgey dir exists
        let budgey_path = &self.budgey_path;
        let read_dir =
            fs::read_dir(budgey_path).map_err(|_| CreateNewPileError::BudgetCouldntBeRead)?;
        // Check if the budget exists
        let one_budget_exists = read_dir
            .filter(|each| match each {
                Ok(v) => v.file_name() == budget_name,
                Err(_) => false,
            })
            .count()
            == 1;

        if !one_budget_exists {
            return Err(CreateNewPileError::OneNamedBudgetDirDoesntExist);
        }
        let pile_name = &pile.name;
        let named_budget_path = budgey_path.to_string() + "/" + budget_name + "/" + pile_name;

        fs::create_dir(&named_budget_path).map_err(|_| CreateNewPileError::CouldntCreatePileDir)?;
        let file_name = pile.name.to_string() + ".json";
        let json_contents = serde_json::to_string(&pile.name)
            .map_err(|_| CreateNewPileError::ConvertingPileToJsonFailed)?;
        fs::write(named_budget_path + "/" + &file_name, json_contents)
            .map_err(|_| CreateNewPileError::WritingJsonFailed)?;

        Ok(())
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
