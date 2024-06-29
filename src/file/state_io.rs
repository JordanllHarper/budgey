use std::fs;

use crate::{budgey_state::BudgeyState, utils::concat_paths, BudgeyConfig};

/// An interface for writing, reading and querying relevant budgey files
pub trait StateIO {
    fn write_budgey_state(&self, state: &BudgeyState) -> anyhow::Result<()>;

    fn read_budgey_state(&self) -> anyhow::Result<BudgeyState, std::io::Error>;

    fn check_state_initialised(&self) -> anyhow::Result<bool>;
}

pub struct StateIOImpl {
    config: BudgeyConfig,
}

impl StateIO for StateIOImpl {
    /// Writes a new budgey state file with the given path and state
    fn write_budgey_state(&self, new_state: &BudgeyState) -> anyhow::Result<()> {
        let budgey_config = &self.config;
        let serialized = serde_json::to_string(new_state)?;
        let check_path_result = fs::read_dir(&budgey_config.root_path);
        if let Err(e) = check_path_result {
            if e.kind() == std::io::ErrorKind::NotFound {
                fs::create_dir_all(&self.config.root_path)?;
            }
        };
        fs::write(
            concat_paths(&budgey_config.root_path, &budgey_config.state_json_name),
            serialized,
        )?;
        Ok(())
    }
    fn read_budgey_state(&self) -> anyhow::Result<BudgeyState, std::io::Error> {
        let config = &self.config;

        let state_path = concat_paths(&config.root_path, &config.state_json_name);
        let read_result = fs::read_to_string(state_path)?;
        let state: BudgeyState = serde_json::from_str(&read_result)?;
        Ok(state)
    }

    fn check_state_initialised(&self) -> anyhow::Result<bool> {
        let check_path_result = match fs::read_dir(&self.config.root_path) {
            Ok(r) => r,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(false);
                }
                return Err(e.into());
            }
        };

        let contains_result = check_path_result
            .into_iter()
            .flatten()
            .flat_map(|file| file.file_name().into_string())
            .any(|each| each.eq(&self.config.state_json_name));

        Ok(contains_result)
    }
}

impl StateIOImpl {
    pub fn new(config: &BudgeyConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}
