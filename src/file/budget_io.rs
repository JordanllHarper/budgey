use std::fs;

use crate::{
    models::budget::Budget,
    utils::{self, create_json_path},
    BudgeyConfig, BudgeyContext,
};

pub trait BudgetIO {
    fn create_new_budget(&self, budget: &Budget) -> anyhow::Result<()>;
    fn get_current_budget(&self, context: &BudgeyContext) -> anyhow::Result<Budget>;
}

pub struct BudgetIOImpl {
    config: BudgeyConfig,
}
impl BudgetIO for BudgetIOImpl {
    fn create_new_budget(&self, budget: &Budget) -> anyhow::Result<()> {
        let budget_path =
            utils::concat_paths(&self.config.root_path, &budget.budget_detail.budget_name);
        if let Err(e) = fs::create_dir(&budget_path) {
            if let std::io::ErrorKind::AlreadyExists = e.kind() {
                println!(
                "It looks like a budget with the name {} already exists. Please choose a different name.",
                budget.budget_detail.budget_name
            );
            }
            return Err(e.into());
        };
        let budget_file_path = create_json_path(&budget_path, &budget.budget_detail.budget_name);
        fs::write(budget_file_path, serde_json::to_string(&budget)?)?;
        Ok(())
    }

    fn get_current_budget(&self, context: &BudgeyContext) -> anyhow::Result<Budget> {
        let current_budget_path = &context.get_current_budget_json_path();
        let current_budget_json = fs::read_to_string(current_budget_path)?;
        let current_budget: Budget = serde_json::from_str(&current_budget_json)?;
        Ok(current_budget)
    }
}

impl BudgetIOImpl {
    pub fn new(config: &BudgeyConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}
