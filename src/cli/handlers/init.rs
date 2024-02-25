use std::io;

use crate::{
    config::local_config::LocalConfig,
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
pub struct InitHandlerImpl {
    local_config: LocalConfig,
}

impl InitHandlerImpl {
    pub fn new(local_config: LocalConfig) -> Self {
        Self { local_config }
    }
}

// TODO: We could do with some better error handling here as this just returns what step failed
// (not what specifically the issue was), this would be good for helping new users if something
// goes wrong
impl InitHandler for InitHandlerImpl {
    fn handle(&self, budget_name: &str) -> anyhow::Result<(), InitHandlerError> {
        create_budgey_dir_if_not_exists(&self.local_config.budgey_dir)
            .map_err(|_| InitHandlerError::CreateBudgeyDirFailed)?;

        create_named_budget_dir(&self.local_config.budgey_dir, budget_name).map_err(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                InitHandlerError::BudgetDirectoryAlreadyExists
            } else {
                InitHandlerError::CreateNamedBudgetDirFailed
            }
        })?;

        // TODO: Create a main pile
        //
        Ok(())
    }
}
