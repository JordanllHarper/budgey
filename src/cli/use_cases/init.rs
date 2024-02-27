use std::io;

use crate::{
    models::budget::Budget,
    repo::budget_repository::{BudgetRepository, BudgetRepositoryImpl},
    utils::{
        error::UseCaseError,
        io::{create_budgey_dir_if_not_exists, create_named_budget_dir},
    },
};

pub enum InitUseCaseError {
    CreateBudgeyDirFailed,
    CreateNamedBudgetDirFailed,
    BudgetDirectoryAlreadyExists,
}
impl UseCaseError for InitUseCaseError {
    fn get_user_message(&self) -> String {
        match self {
            InitUseCaseError::CreateBudgeyDirFailed => {
                String::from("Something went wrong when trying to initialise the budgey directory.")
            }

            InitUseCaseError::CreateNamedBudgetDirFailed => String::from(
                "Something went wrong when trying to initialise the new budget directory.",
            ),
            InitUseCaseError::BudgetDirectoryAlreadyExists => {
                String::from("That budget name was already found in your budgey directory.")
            }
        }
    }
}

pub trait InitUseCase {
    fn handle(&self, repo_name: &str) -> anyhow::Result<(), InitUseCaseError>;
}
pub struct InitUseCaseImpl<T>
where
    T: BudgetRepository,
{
    budgey_directory: String,
    budget_repo: T,
}

impl<T> InitUseCaseImpl<T>
where
    T: BudgetRepository,
{
    pub fn new(budgey_directory: &str, budget_repo: T) -> Self {
        Self {
            budgey_directory: budgey_directory.to_string(),
            budget_repo,
        }
    }
}

// TODO: We could do with some better irror handling here as this just returns what step failed
// (not what specifically the issue was), this would be good for helping new users if something
// goes wrong
impl<T> InitUseCase for InitUseCaseImpl<T>
where
    T: BudgetRepository,
{
    fn handle(&self, budget_name: &str) -> anyhow::Result<(), InitUseCaseError> {
        create_budgey_dir_if_not_exists(&self.budgey_directory)
            .map_err(|_| InitUseCaseError::CreateBudgeyDirFailed)?;

        create_named_budget_dir(&self.budgey_directory, budget_name).map_err(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                return InitUseCaseError::BudgetDirectoryAlreadyExists;
            }
            InitUseCaseError::CreateNamedBudgetDirFailed
        })?;
        // TODO: Create budget
        self.budget_repo.create_new_budget(Budget::new(budget_name));

        Ok(())
    }
}
