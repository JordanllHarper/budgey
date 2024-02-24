use crate::{
    config::local_config::LocalConfig,
    utils::{
        error::HandlerError,
        io::{create_budgey_dir_if_not_exists, create_named_budget_dir_if_not_exists},
    },
};

pub enum InitHandlerError {
    CreateBudgeyDirFailed,
    CreateNamedBudgetDirFailed,
}
impl HandlerError for InitHandlerError {
    fn get_user_message(&self) -> String {
        match self {
            InitHandlerError::CreateBudgeyDirFailed => {
                String::from("Couldn't create the budgey directory. Exiting...")
            }

            InitHandlerError::CreateNamedBudgetDirFailed => {
                String::from("Couldn't create the new budget's directory. Exiting...")
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
        if let Err(_) = create_budgey_dir_if_not_exists(&self.local_config.budgey_dir) {
            return Err(InitHandlerError::CreateBudgeyDirFailed);
        }

        match create_named_budget_dir_if_not_exists(&self.local_config.budgey_dir, budget_name) {
            Err(_) => Err(InitHandlerError::CreateNamedBudgetDirFailed),
            Ok(_) => Ok(()),
        }
        // TODO: Create a main pile
    }
}
