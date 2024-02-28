use std::{fs, io, path::Path};

use crate::{
    io_operations::io_operations::io_operations::{
        create_budgey_dir_if_not_exists, create_named_budget_dir,
    },
    models::budget::Budget,
};
pub enum CreateNewBudgetError {
    CreateBudgeyDirFailed,
    CreateNamedBudgetDirFailed,
    BudgetDirectoryAlreadyExists,
    CouldntConvertToJson,
    CouldntWriteJson,
}

pub enum BudgetRepositoryError {
    CreateNewBudgetError(CreateNewBudgetError),
}

pub trait BudgetRepository {
    fn create_new_budget(&self, budget: Budget) -> anyhow::Result<(), CreateNewBudgetError>;
    fn get_all_budgets(&self) -> anyhow::Result<Vec<String>, BudgetRepositoryError>;
    fn delete_budget(&self, budget_name: &str) -> anyhow::Result<(), BudgetRepositoryError>;
}

pub struct BudgetRepositoryImpl {
    budgey_directory: String,
}

impl BudgetRepositoryImpl {
    pub fn new(budgey_directory: &str) -> Self {
        Self {
            budgey_directory: budgey_directory.to_string(),
        }
    }
}

impl BudgetRepository for BudgetRepositoryImpl {
    fn create_new_budget(&self, budget: Budget) -> Result<(), CreateNewBudgetError> {
        let budget_name = &budget.budget_detail.budget_name;
        let budgey_dir = &self.budgey_directory;

        create_budgey_dir_if_not_exists(budgey_dir)
            .map_err(|_| CreateNewBudgetError::CreateBudgeyDirFailed)?;

        let budget_path =
            create_named_budget_dir(&self.budgey_directory, budget_name).map_err(|e| {
                if e.kind() == io::ErrorKind::AlreadyExists {
                    return CreateNewBudgetError::BudgetDirectoryAlreadyExists;
                }
                CreateNewBudgetError::CreateNamedBudgetDirFailed
            })? + budget_name
                + ".json";

        let detail_json = serde_json::to_string(&budget.budget_detail)
            .map_err(|_| CreateNewBudgetError::CouldntConvertToJson)?;
        fs::write(Path::new(&budget_path), detail_json)
            .map_err(|_| CreateNewBudgetError::CouldntWriteJson)?;
        Ok(())
    }

    fn get_all_budgets(&self) -> Result<Vec<std::string::String>, BudgetRepositoryError> {
        todo!()
    }

    fn delete_budget(&self, budget_name: &str) -> Result<(), BudgetRepositoryError> {
        todo!()
    }
}
