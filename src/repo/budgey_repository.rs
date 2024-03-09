use std::{fs, io::ErrorKind};

#[derive(Debug)]
pub enum InitBudgeyError {
    BudgeyAlreadyExists,
    BudgeyCreationFailed,
}
pub enum CheckBudgeyExistsError {
    SomethingWentWrong,
}
pub fn init_budgey(budgey_path: &str) -> anyhow::Result<(), InitBudgeyError> {
    fs::create_dir(&budgey_path).map_err(|e| {
        if let ErrorKind::AlreadyExists = e.kind() {
            InitBudgeyError::BudgeyAlreadyExists
        } else {
            InitBudgeyError::BudgeyCreationFailed
        }
    })?;
    Ok(())
}

fn check_budgey_exists(budgey_path: &str) -> anyhow::Result<bool, CheckBudgeyExistsError> {
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
