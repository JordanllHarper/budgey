use std::{fs, io, path::Path, result::Result::Ok};

/// Creates the budget directory where the budget information will be stored.
/// Unlike below, we do care if the directory already exists here, as that means the init command
/// failed
pub fn create_named_budget_dir(
    root_dir: &str,
    budget_name: &str,
) -> anyhow::Result<(), std::io::Error> {
    let dir = format!("/{root_dir}/{budget_name}");
    let path = Path::new(&dir);
    fs::create_dir(path)?;
    Ok(())
}
/// Creates the budgey root directory if it is not already created.
/// We don't want to return an error if the file already exists (as it should just do nothing in
/// that case).
pub fn create_budgey_dir_if_not_exists(path_to_dir: &str) -> anyhow::Result<(), std::io::Error> {
    let path = Path::new(path_to_dir);
    let result = fs::create_dir(path);
    if let Err(e) = result {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return anyhow::Result::Err(e);
        }
    }
    Ok(())
}
