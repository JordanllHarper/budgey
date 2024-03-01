use crate::{
    models::{budget::Budget, pile::Pile},
    repo::{
        budget_repository::{BudgetRepository, CreateNewBudgetError},
        pile_repository::{CreateNewPileError, PileRepository},
    },
    utils::error::GenericError,
};

pub enum InitError {
    CreateBudgetError(CreateNewBudgetError),
    CreatePileError(CreateNewPileError),
}
impl InitError {
    pub fn new_from_budget_error(e: CreateNewBudgetError) -> InitError {
        InitError::CreateBudgetError(e)
    }

    pub fn new_from_pile_error(e: CreateNewPileError) -> InitError {
        InitError::CreatePileError(e)
    }
}

impl GenericError for InitError {
    fn get_user_message(&self) -> String {
        match self {
            InitError::CreateBudgetError(e) => match e {
                CreateNewBudgetError::CreateBudgeyDirFailed => String::from(
                    "Something went wrong when trying to initialise the budgey directory.",
                ),

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
                CreateNewPileError::BudgeyDirectoryDoesntExist => {
                    String::from("The budgey directory doesn't exist.")
                }
                CreateNewPileError::OneNamedBudgetDirDoesntExist => {
                    String::from("The named budget directory doesn't exist.")
                }
                CreateNewPileError::BudgetCouldntBeRead => {
                    String::from("The budget couldn't be read.")
                }
                CreateNewPileError::CouldntCreatePileDir => {
                    String::from("The pile directory couldn't be created.")
                }
                CreateNewPileError::ConvertingPileToJsonFailed => {
                    String::from("Creating the json pile failed.")
                }
                CreateNewPileError::WritingJsonFailed => {
                    String::from("Writing the json pile failed.")
                }
            },
        }
    }
}

pub trait InitUseCase {
    fn handle(&self, repo_name: &str) -> anyhow::Result<(), InitError>;
}
pub struct InitUseCaseImpl<T, R>
where
    T: BudgetRepository,
    R: PileRepository,
{
    budget_repo: T,
    pile_repo: R,
}

impl<T, R> InitUseCaseImpl<T, R>
where
    T: BudgetRepository,
    R: PileRepository,
{
    pub fn new(budget_repo: T, pile_repo: R) -> Self {
        Self {
            budget_repo,
            pile_repo,
        }
    }
}

// TODO: We could do with some better irror handling here as this just returns what step failed
// (not what specifically the issue was), this would be good for helping new users if something
// goes wrong
impl<T, R> InitUseCase for InitUseCaseImpl<T, R>
where
    T: BudgetRepository,
    R: PileRepository,
{
    fn handle(&self, budget_name: &str) -> Result<(), InitError> {
        self.budget_repo
            .create_new_budget(Budget::new(budget_name))
            .map_err(|e| InitError::new_from_budget_error(e))?;

        self.pile_repo
            .create_new_pile(Pile::default(), budget_name)
            .map_err(|e| InitError::new_from_pile_error(e))?;

        Ok(())
    }
}
