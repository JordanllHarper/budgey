use std::io;

use crate::{
    config::local_config::LocalConfig,
    models::{budget::Budget, pile::Pile},
    repo::budget_repository::{BudgetRepository, BudgetRepositoryImpl},
    utils::{
        error::HandlerError,
        io::{create_budgey_dir_if_not_exists, create_named_budget_dir},
    },
};

pub enum InitHandlerError {
    CreateBudgeyDirFailed,
    CreateNamedBudgetDirFailed,
    BudgetDirectoryAlreadyExists,
}
impl HandlerError for InitHandlerError {
    fn get_user_message(&self) -> String {
        match self {
            InitHandlerError::CreateBudgeyDirFailed => {
                String::from("Something went wrong when trying to initialise the budgey directory.")
            }

            InitHandlerError::CreateNamedBudgetDirFailed => String::from(
                "Something went wrong when trying to initialise the new budget directory.",
            ),
            InitHandlerError::BudgetDirectoryAlreadyExists => {
                String::from("That budget name was already found in your budgey directory.")
            }
        }
    }
}

pub trait InitHandler {
    fn handle(&self, repo_name: &str) -> anyhow::Result<(), InitHandlerError>;
}
pub struct InitHandlerImpl<T>
where
    T: BudgetRepository,
{
    budgey_directory: String,
    budget_repo: T,
}

impl<T> InitHandlerImpl<T>
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
impl InitHandler for InitHandlerImpl<BudgetRepositoryImpl> {
    fn handle(&self, budget_name: &str) -> anyhow::Result<(), InitHandlerError> {
        create_budgey_dir_if_not_exists(&self.budgey_directory)
            .map_err(|_| InitHandlerError::CreateBudgeyDirFailed)?;

        create_named_budget_dir(&self.budgey_directory, budget_name).map_err(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                return InitHandlerError::BudgetDirectoryAlreadyExists;
            }
            InitHandlerError::CreateNamedBudgetDirFailed
        })?;
        // TODO: Create budget
        self.budget_repo
            .create_new_budget(Budget::new(budget_name, Pile::new_with_main()));

        Ok(())
    }
}
