use crate::{
    handling::{
        budget_handling::CreateNewBudgetError, budgey_handling::InitBudgeyError,
        pile_handling::CreateNewPileError,
    },
    models::{budget::Budget, pile::Pile},
    InitBudgetData,
};

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    CreateBudgeyError(InitBudgeyError),
    CreateBudgetError(CreateNewBudgetError),
    CreatePileError(CreateNewPileError),
}

pub struct InitBudgetData {
    budgey_path: String,
    budget_name: String,
}

impl InitBudgetData {
    pub fn new(budgey_path: String, budget_name: String) -> Self {
        Self {
            budgey_path,
            budget_name,
        }
    }
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

pub fn handle_init(
    init_budget_data: InitBudgetData,
    init_budgey: fn(budgey_path: &str) -> Result<(), InitBudgeyError>,
    create_new_budget: fn(
        budgey_directory: &str,
        budget: Budget,
    ) -> Result<(), CreateNewBudgetError>,
    create_new_pile: fn(
        pile: Pile,
        budget_name: &str,
        budgey_path: &str,
    ) -> Result<(), CreateNewPileError>,
) -> anyhow::Result<(), InitError> {
    println!("Budgey path: {}", budgey_path);
    println!("Creating new budget under name: {}", budget_name);
    let result = init_budgey(budgey_path);
    if let Err(e) = result {
        match e {
            InitBudgeyError::BudgeyAlreadyExists => {
                // TODO: This should be returned to allow for testing
                println!("Found budgey directory already. Creating new budget under name...");
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
#[cfg(test)]
pub mod init_tests {
    use crate::handling::{
        budget_handling::CreateNewBudgetError, budgey_handling::InitBudgeyError,
        pile_handling::CreateNewPileError,
    };

    #[test]
    pub fn handle_init_success() {
        let budgey_path = "test_budgey";
        let budget_name = "test_budget";
        let result = super::handle_init(
            budgey_path,
            budget_name,
            |_| Ok(()),
            |_, _| Ok(()),
            |_, _, _| Ok(()),
        );
        assert!(result.is_ok());
    }

    #[test]
    pub fn handle_init_success_with_budgey_already_exists() {
        let budgey_path = "test_budgey";
        let budget_name = "test_budget";
        let result = super::handle_init(
            budgey_path,
            budget_name,
            |_| Err(InitBudgeyError::BudgeyAlreadyExists),
            |_, _| Ok(()),
            |_, _, _| Ok(()),
        );
        assert!(result.is_ok());
    }

    #[test]
    pub fn handle_init_failure_budgey_creation_fails() {
        let budgey_path = "test_budgey";
        let budget_name = "test_budget";
        let result = super::handle_init(
            budgey_path,
            budget_name,
            |_| Err(InitBudgeyError::BudgeyCreationFailed),
            |_, _| Ok(()),
            |_, _, _| Ok(()),
        );
        assert!(result.is_err());
    }

    #[test]
    pub fn handle_init_failure_create_budget_fails() {
        let budgey_path = "test_budgey";
        let budget_name = "test_budget";
        let result = super::handle_init(
            budgey_path,
            budget_name,
            |_| Ok(()),
            |_, _| Err(CreateNewBudgetError::CreateNamedBudgetDirFailed),
            |_, _, _| Ok(()),
        );
        assert!(result.is_err());
    }
    #[test]
    pub fn handle_init_failure_pile_creation_fails() {
        let budgey_path = "test_budgey";
        let budget_name = "test_budget";
        let result = super::handle_init(
            budgey_path,
            budget_name,
            |_| Ok(()),
            |_, _| Ok(()),
            |_, _, _| Err(CreateNewPileError::CouldntCreatePileDirectory),
        );
        assert!(result.is_err());
    }
}
