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
