use std::fs;

use log::{error, trace};

use crate::{
    models::budget::Budget,
    utils::{self, create_json_path},
    BudgeyConfig,
};

pub trait BudgetIO {
    fn create_new_budget(&self, budget: &Budget) -> anyhow::Result<()>;
}

pub struct BudgetIOImpl {
    config: BudgeyConfig,
}
impl BudgetIO for BudgetIOImpl {
    fn create_new_budget(&self, budget: &Budget) -> anyhow::Result<()> {
        let budget_path =
            utils::concat_paths(&self.config.root_path, &budget.budget_detail.budget_name);
        trace!("Creating new budget");
        match fs::create_dir(&budget_path) {
            Ok(_) => {}
            Err(e) => {
                if let std::io::ErrorKind::AlreadyExists = e.kind() {
                    error!(
                        "Budget {} already exists at: {}",
                        budget.budget_detail.budget_name, budget_path
                    );
                    println!(
                    "It looks like a budget with the name {} already exists. Please choose a different name.",
                    budget.budget_detail.budget_name
                );
                    return Err(e.into());
                }
                error!("Error creating budget directory at: {}", budget_path);
                return Err(e.into());
            }
        };
        let budget_file_path = create_json_path(&budget_path, &budget.budget_detail.budget_name);
        match fs::write(
            budget_file_path,
            match serde_json::to_string(&budget) {
                Ok(it) => it,
                Err(e) => {
                    error!("Error serializing budget: {:?}", e);
                    return Err(e.into());
                }
            },
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.into());
            }
        };
        Ok(())
    }
}

impl BudgetIOImpl {
    pub fn new(config: &BudgeyConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}
