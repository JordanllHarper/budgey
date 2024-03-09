use crate::{
    handling::{
        budget_handling::{create_new_budget, CreateNewBudgetError},
        budgey_handling::{init_budgey, InitBudgeyError},
        pile_handling::{create_new_pile, CreateNewPileError},
    },
    models::{budget::Budget, pile::Pile},
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

pub fn handle_init(budgey_path: &str, budget_name: &str) -> anyhow::Result<()> {
    let result = init_budgey(budgey_path);
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

    create_new_budget(budgey_path, Budget::new(budget_name))
        .map_err(|e| InitError::new_from_budget_error(e))?;
    create_new_pile(Pile::default(), budget_name, budgey_path)
        .map_err(|e| InitError::new_from_pile_error(e))?;

    Ok(())
}
