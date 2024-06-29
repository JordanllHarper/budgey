use std::{fs, io::ErrorKind};

use colored::Colorize;
use log::{error, trace};

use crate::{budgey_state::BudgeyState, utils::concat_paths, BudgeyConfig};

/// An interface for writing, reading and querying relevant budgey files
pub trait StateIO {
    fn write_budgey_state(&self, state: &BudgeyState) -> anyhow::Result<()>;

    fn read_budgey_state(&self) -> anyhow::Result<BudgeyState>;

    fn check_state_initialised(&self) -> anyhow::Result<bool>;
}

pub struct StateIOImpl {
    config: BudgeyConfig,
}

impl StateIO for StateIOImpl {
    /// Writes a new budgey state file with the given path and state
    fn write_budgey_state(&self, new_state: &BudgeyState) -> anyhow::Result<()> {
        let budgey_config = &self.config;
        trace!("Writing budgey state");
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
    fn read_budgey_state(&self) -> anyhow::Result<BudgeyState> {
        trace!("Getting budgey state");
        let config = &self.config;

        let state_path = concat_paths(&config.root_path, &config.state_json_name);
        let read_result = match fs::read_to_string(state_path) {
            Ok(v) => v,
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    // TODO: Extract this out to callers
                    println!(
                        "Couldn't find the state file.\n\nPlease run {}",
                        "`budgey init`".green()
                    );
                    return Err(e.into());
                } else {
                    error!("Error reading budgey state: {:?}", e);
                    return Err(e.into());
                }
            }
        };
        let state: BudgeyState = serde_json::from_str(&read_result)?;
        Ok(state)
    }

    fn check_state_initialised(&self) -> anyhow::Result<bool> {
        trace!("Checking if budgey state is initialised");
        let check_path_result = match fs::read_dir(&self.config.root_path) {
            Ok(r) => r,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(false);
                }
                error!("Error reading budgey path: {:?}", e);
                return Err(e.into());
            }
        };

        let contains_result = check_path_result
            .into_iter()
            .flatten()
            .flat_map(|file| file.file_name().into_string())
            .any(|each| each.eq(&self.config.state_json_name));

        trace!("Budgey initialised: {}", contains_result);

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
