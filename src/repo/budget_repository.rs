use std::{fs, io::ErrorKind};

use crate::models::budget::Budget;

pub enum CreateNewBudgetError {
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
        let budget_path = format!("{}/{}", self.budgey_directory, budget_name);
        fs::create_dir(&budget_path).map_err(|e| {
            if let ErrorKind::AlreadyExists = e.kind() {
                CreateNewBudgetError::BudgetDirectoryAlreadyExists
            } else {
                CreateNewBudgetError::CreateNamedBudgetDirFailed
            }
        })?;
        let json_file = format!("{}/{}.json", &budget_path, budget_name);
        fs::write(json_file, serde_json::to_string(&budget).unwrap())
            .map_err(|_| CreateNewBudgetError::CouldntWriteJson)?;

        todo!()
    }

    fn get_all_budgets(&self) -> Result<Vec<std::string::String>, BudgetRepositoryError> {
        todo!()
    }

    fn delete_budget(&self, budget_name: &str) -> Result<(), BudgetRepositoryError> {
        todo!()
    }
}
