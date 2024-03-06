use crate::repo::pile_repository::{CreateNewPileError, PileRepository};
use crate::{
    models::{budget::Budget, pile::Pile},
    repo::{
        budget_repository::{BudgetRepository, CreateNewBudgetError},
        budgey_repository::{BudgeyRepository, InitBudgeyError},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    CreateBudgeyError(InitBudgeyError),
    CreateBudgetError(CreateNewBudgetError),
    CreatePileError(CreateNewPileError),
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            InitError::CreateBudgetError(init_error) => match init_error {
                CreateNewBudgetError::CreateNamedBudgetDirFailed => String::from(
                    "Something went wrong when trying to initialise the new budget directory.",
                ),
                CreateNewBudgetError::BudgetDirectoryAlreadyExists => {
                    String::from("That budget name was already found in your budgey directory.")
                }
                CreateNewBudgetError::CouldntConvertToJson => {
                    String::from("Something went wrong with saving the budget.")
                }
                CreateNewBudgetError::CouldntWriteJson => {
                    String::from("Something went wrong with writing the json.")
                }
            },
            InitError::CreatePileError(e) => match e {
                CreateNewPileError::PileDirectoryAlreadyExists => {
                    String::from("The pile has already been made.")
                }
                CreateNewPileError::CouldntCreatePileDirectory => {
                    String::from("The pile directory couldn't be created.")
                }
                CreateNewPileError::CouldntWriteJson => {
                    String::from("Something went wrong with writing the json.")
                }
            },
            InitError::CreateBudgeyError(e) => match e {
                InitBudgeyError::BudgeyAlreadyExists => {
                    String::from("The budgey directory already exists.")
                }
                InitBudgeyError::BudgeyCreationFailed => {
                    String::from("The budgey directory couldn't be created.")
                }
            },
        };
        f.write_str(&message)?;
        Ok(())
    }
}

impl InitError {
    pub fn new_from_budgey_error(e: InitBudgeyError) -> InitError {
        InitError::CreateBudgeyError(e)
    }
    pub fn new_from_budget_error(e: CreateNewBudgetError) -> InitError {
        InitError::CreateBudgetError(e)
    }

    pub fn new_from_pile_error(e: CreateNewPileError) -> InitError {
        InitError::CreatePileError(e)
    }
}

pub trait InitUseCase {
    fn handle(&self, repo_name: &str) -> anyhow::Result<()>;
}
pub struct InitUseCaseImpl<'a> {
    budget_repo: &'a dyn BudgetRepository,
    pile_repo: &'a dyn PileRepository,
    budgey_repo: &'a dyn BudgeyRepository,
}

impl<'a> InitUseCaseImpl<'a> {
    pub fn new(
        budget_repo: &'a dyn BudgetRepository,
        pile_repo: &'a dyn PileRepository,
        budgey_repo: &'a dyn BudgeyRepository,
    ) -> Self {
        Self {
            budget_repo,
            pile_repo,
            budgey_repo,
        }
    }
}

// TODO: We could do with some better error handling here as this just returns what step failed
// (not what specifically the issue was), this would be good for helping new users if something
// goes wrong
impl<'a> InitUseCase for InitUseCaseImpl<'a> {
    fn handle(&self, budget_name: &str) -> anyhow::Result<()> {
        let result = self.budgey_repo.init_budgey();
        if let Err(e) = result {
            match e {
                InitBudgeyError::BudgeyAlreadyExists => {
                    println!("Found budgey directory already. Creating new budget under name...")
                }
                InitBudgeyError::BudgeyCreationFailed => {
                    return Err(InitError::new_from_budgey_error(e).into());
                }
            }
        }

        self.budget_repo
            .create_new_budget(Budget::new(budget_name))
            .map_err(|e| InitError::new_from_budget_error(e))?;
        self.pile_repo
            .create_new_pile(Pile::default(), budget_name)
            .map_err(|e| InitError::new_from_pile_error(e))?;

        Ok(())
    }
}
