use std::{fs, path::Path};

pub trait FileHandler {
    fn create_directory(path_to_directory: &Path) -> anyhow::Result<()>;
    fn write_to_file(path_to_file: &Path, contents: &str) -> anyhow::Result<()>;
    fn read_from_file(path_to_dir: &Path, file_name: &str) -> anyhow::Result<String>;
}
pub struct FileHandlerImpl {
    budgey_dir: String,
}

impl FileHandler for FileHandlerImpl {
    fn create_directory(path_to_directory: &Path) -> anyhow::Result<()> {
        fs::create_dir(path_to_directory)?;
        Ok(())
    }

    fn write_to_file(path_to_file: &Path, contents: &str) -> anyhow::Result<()> {
        fs::write(path_to_file, contents)?;
        Ok(())
    }

    fn read_from_file(path_to_dir: &Path, file_name: &str) -> anyhow::Result<String> {
        todo!()
    }
}

impl FileHandlerImpl {
    pub fn new(budgey_dir: &str) -> Self {
        Self {
            budgey_dir: budgey_dir.to_string(),
        }
    }
}
