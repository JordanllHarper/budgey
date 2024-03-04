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
        todo!()
    }

    fn get_all_budgets(&self) -> Result<Vec<std::string::String>, BudgetRepositoryError> {
        todo!()
    }

    fn delete_budget(&self, budget_name: &str) -> Result<(), BudgetRepositoryError> {
        todo!()
    }
}
