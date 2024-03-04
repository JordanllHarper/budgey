use std::{fs, io::ErrorKind};

pub enum InitBudgeyError {
    BudgeyAlreadyExists,
    BudgeyCreationFailed,
}
pub enum CheckBudgeyExistsError {
    SomethingWentWrong,
}
pub trait BudgeyRepository {
    fn init_budgey(&self) -> anyhow::Result<(), InitBudgeyError>;
    fn check_budgey_exists(&self) -> anyhow::Result<bool, CheckBudgeyExistsError>;
}
pub struct BudgeyRepositoryImpl {
    budgey_dir: String,
}

impl BudgeyRepositoryImpl {
    pub fn new(budgey_dir: &str) -> Self {
        Self {
            budgey_dir: budgey_dir.to_string(),
        }
    }
}

fn create_budgey_path(root: &str) -> String {
    format!("/{root}/budgey/")
}

impl BudgeyRepository for BudgeyRepositoryImpl {
    fn init_budgey(&self) -> anyhow::Result<(), InitBudgeyError> {
        let budgey_path = create_budgey_path(&self.budgey_dir);
        fs::create_dir(&budgey_path).map_err(|e| {
            if let ErrorKind::AlreadyExists = e.kind() {
                InitBudgeyError::BudgeyAlreadyExists
            } else {
                InitBudgeyError::BudgeyCreationFailed
            }
        })?;
        Ok(())
    }

    fn check_budgey_exists(&self) -> anyhow::Result<bool, CheckBudgeyExistsError> {
        let budgey_path = create_budgey_path(&self.budgey_dir);
        let result = fs::metadata(budgey_path);
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    Ok(false)
                } else {
                    Err(CheckBudgeyExistsError::SomethingWentWrong)
                }
            }
        }
    }
}
