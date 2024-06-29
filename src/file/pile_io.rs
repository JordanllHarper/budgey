use std::fs;

use crate::{
    budget_management::update_budget,
    models::{budget::Budget, pile::Pile},
    utils::{concat_paths, create_json_path},
    BudgeyContext,
};

pub trait PileIO {
    fn get_current_pile(&self, current_budget: &Budget) -> anyhow::Result<Pile>;
    fn update_pile(&self, new_pile: &Pile) -> anyhow::Result<()>;
    fn get_pile(&self, pile_name: &str) -> anyhow::Result<Pile>;
    fn maybe_get_pile(
        &self,
        pile_name: &str,
        current_budget: &Budget,
    ) -> anyhow::Result<Option<Pile>>;
    fn create_new_pile(&self, pile: &Pile) -> anyhow::Result<(), std::io::Error>;
    fn delete_pile(&self, pile_name: &str, current_budget: &Budget) -> anyhow::Result<()>;
}
pub struct PileIOImpl {
    context: BudgeyContext,
}

impl PileIOImpl {
    pub fn new(context: &BudgeyContext) -> Self {
        Self {
            context: context.clone(),
        }
    }
}

impl PileIO for PileIOImpl {
    fn get_current_pile(&self, current_budget: &Budget) -> anyhow::Result<Pile> {
        self.get_pile(&current_budget.current_pile_name)
    }

    fn update_pile(&self, new_pile: &Pile) -> anyhow::Result<()> {
        let pile_path = concat_paths(
            &self.context.get_current_budget_path(),
            &new_pile.get_name(),
        );
        let pile_json_path = create_json_path(&pile_path, &new_pile.get_name());
        fs::write(pile_json_path, serde_json::to_string(&new_pile)?)?;
        Ok(())
    }

    fn get_pile(&self, pile_name: &str) -> anyhow::Result<Pile> {
        let pile_path = concat_paths(&self.context.get_current_budget_path(), pile_name);
        let pile_json_path = create_json_path(&pile_path, pile_name);
        let pile_json = fs::read_to_string(pile_json_path)?;
        Ok(serde_json::from_str::<Pile>(&pile_json)?)
    }

    /// Gets a pile if it exists in the current budget.
    fn maybe_get_pile(
        &self,
        pile_name: &str,
        current_budget: &Budget,
    ) -> anyhow::Result<Option<Pile>> {
        let in_budget_ledger = current_budget.pile_names.contains(&pile_name.into());
        if !in_budget_ledger {
            return Ok(None);
        }

        let pile = self.get_pile(pile_name)?;
        Ok(Some(pile))
    }

    fn create_new_pile(&self, pile: &Pile) -> std::result::Result<(), std::io::Error> {
        let current_budget_path = self.context.get_current_budget_path();
        let pile_name = pile.get_name();
        let pile_directory_path = concat_paths(&current_budget_path, &pile_name);
        fs::create_dir(pile_directory_path)?;
        let pile_file_path =
            create_json_path(&concat_paths(&current_budget_path, &pile_name), &pile_name);
        fs::write(pile_file_path, serde_json::to_string(&pile)?)?;

        Ok(())
    }

    fn delete_pile(&self, pile_name: &str, current_budget: &Budget) -> anyhow::Result<()> {
        let pile_path = concat_paths(&self.context.get_current_budget_path(), pile_name);
        fs::remove_dir_all(pile_path)?;
        let new_budget = current_budget.delete_pile(pile_name);
        update_budget(&self.context.get_current_budget_path(), &new_budget)?;
        Ok(())
    }
}
