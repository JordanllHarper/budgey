use crate::{
    models::budget::Budget,
    repo::{
        budget_repository::{BudgetRepository, CreateNewBudgetError},
        pile_repository::PileRepository,
    },
    utils::error::UseCaseError,
};

impl UseCaseError for CreateNewBudgetError {
    fn get_user_message(&self) -> String {
        match self {
            CreateNewBudgetError::CreateBudgeyDirFailed => {
                String::from("Something went wrong when trying to initialise the budgey directory.")
            }

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
        }
    }
}

pub trait InitUseCase {
    fn handle(&self, repo_name: &str) -> anyhow::Result<(), CreateNewBudgetError>;
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
    fn handle(&self, budget_name: &str) -> Result<(), CreateNewBudgetError> {
        self.budget_repo
            .create_new_budget(Budget::new(budget_name))?;

        Ok(())
    }
}
