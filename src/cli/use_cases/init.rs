use crate::{
    models::{budget::Budget, pile::Pile},
    repo::{
        budget_repository::{BudgetRepository, CreateNewBudgetError},
        budgey_repository::{BudgeyRepository, InitBudgeyError},
        pile_repository::{CreateNewPileError, PileRepository},
    },
    utils::error::GenericError,
};

pub enum InitError {
    CreateBudgeyError(InitBudgeyError),
    CreateBudgetError(CreateNewBudgetError),
    CreatePileError(CreateNewPileError),
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

impl GenericError for InitError {
    fn get_user_message(&self) -> String {
        match self {
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
                CreateNewPileError::BudgeyDirectoryDoesntExist => todo!(),
            },
            InitError::CreateBudgeyError(e) => match e {
                InitBudgeyError::BudgeyAlreadyExists => {
                    String::from("The budgey directory already exists.")
                }
                InitBudgeyError::BudgeyCreationFailed => {
                    String::from("The budgey directory couldn't be created.")
                }
            },
        }
    }
}

pub trait InitUseCase {
    fn handle(&self, repo_name: &str) -> anyhow::Result<(), InitError>;
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

// TODO: We could do with some better irror handling here as this just returns what step failed
// (not what specifically the issue was), this would be good for helping new users if something
// goes wrong
impl<'a> InitUseCase for InitUseCaseImpl<'a> {
    fn handle(&self, budget_name: &str) -> Result<(), InitError> {
        let result = self.budgey_repo.init_budgey();
        if let Err(e) = result {
            match e {
                InitBudgeyError::BudgeyAlreadyExists => {
                    println!("Found budgey directory already. Creating new budget under name...")
                }
                InitBudgeyError::BudgeyCreationFailed => {
                    return Err(InitError::new_from_budgey_error(e));
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
