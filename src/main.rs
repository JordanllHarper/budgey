use crate::{
    budgey_state::BudgeyState,
    models::record_transaction::{Record, Transaction, TransactionType},
};
use anyhow::Ok;
use budgey_cli::Commands;
use clap::Parser;
use log::{info, trace};
use utils::{concat_paths, create_json_file_name};

mod budget_management;
mod budgey_cli;
mod budgey_state;
mod handle_budget;
mod handle_init;
mod handle_pile;
mod models;
mod pile_management;
mod utils;

#[derive(Debug, Clone)]
pub struct BudgeyContext {
    budgey_config: BudgeyConfig,
    state: BudgeyState,
}

#[derive(Debug, Clone)]
pub struct BudgeyConfig {
    budgey_path: String,
    state_json_name: String,
}

impl BudgeyConfig {
    pub fn new(budgey_path: &str, state_json_name: &str) -> Self {
        Self {
            budgey_path: budgey_path.to_string(),
            state_json_name: state_json_name.to_string(),
        }
    }
    pub fn get_state_path(&self) -> String {
        concat_paths(&self.budgey_path, &self.state_json_name)
    }
    pub fn get_budget_path(&self, budget_name: &str) -> String {
        concat_paths(&self.budgey_path, budget_name)
    }
}

impl BudgeyContext {
    pub fn new(state: &BudgeyState, budgey_config: &BudgeyConfig) -> Self {
        Self {
            budgey_config: budgey_config.clone(),
            state: state.clone(),
        }
    }
    pub fn get_current_budget_path(&self) -> String {
        let current_budget = &self.state.current_focused_budget_name;
        let budgey_path = &self.budgey_config.budgey_path;
        concat_paths(budgey_path, current_budget)
    }
    pub fn get_current_budget_json_path(&self) -> String {
        concat_paths(
            &self.get_current_budget_path(),
            &create_json_file_name(&self.state.current_focused_budget_name),
        )
    }

    pub fn update_state(&self, new_state: &BudgeyState) -> Self {
        Self::new(new_state, &self.budgey_config)
    }
}

fn main() -> anyhow::Result<()> {
    simple_logger::init().unwrap();

    let home = env!("HOME").to_string();
    info!("Home environment initialised: {}", home);

    let budgey_path = format!("{}{}", home, "/budgey");
    info!("Budgey path: {}", budgey_path);

    let budgey_state_json_name = create_json_file_name("budgey_state");
    let budgey_state_path = concat_paths(&budgey_path, &budgey_state_json_name);
    info!("Budgey state path created: {}", budgey_state_path);

    let budgey_config = BudgeyConfig::new(&budgey_path, &budgey_state_json_name);
    info!("Budgey config created:\n {:#?}", budgey_config);

    info!("Parsing CLI arguments");
    let args = budgey_cli::BudgeyCLI::parse();
    trace!("Parsed CLI arguments: {:#?}", args);

    let _ = match args {
        budgey_cli::BudgeyCLI::Init { name } => handle_init::handle_init(budgey_config, &name),
        budgey_cli::BudgeyCLI::Subcommands(c) => {
            let context = BudgeyContext::new(
                &budgey_state::get_budgey_state(&budgey_state_path)?,
                &budgey_config,
            );

            handle_subcommands(&context, c)
        }
    };
    Ok(())
}

fn handle_subcommands(context: &BudgeyContext, command: Commands) -> anyhow::Result<()> {
    match command {
        budgey_cli::Commands::Budget { subcommand } => {
            if let Some(sub) = &subcommand {
                handle_budget::handle_budget_subcommand(&context, sub.clone())
            } else {
                let current_budget = budget_management::get_current_budget_name(&context)?;
                println!("Current budget: {:?}", current_budget);
                Ok(())
            }
        }
        budgey_cli::Commands::Pile { subcommand } => {
            if let Some(sub) = &subcommand {
                return handle_pile::handle_pile_subcommand(context, sub.clone());
            } else {
                let current_pile = pile_management::get_current_pile(&context)?;
                println!("Current pile: {}", current_pile.get_name());
            }
            Ok(())
        }
        budgey_cli::Commands::Add { amount } => {
            trace!("Adding to pile: amount: {:?}", amount);
            let new_pile = update_pile_with_action(&context, |pile| {
                Ok(pile.add_transaction(&Transaction::new(TransactionType::Add, amount)))
            })?;

            println!(
                "Staged transaction of {}. Pile now at: {}",
                amount, new_pile.current_balance
            );
            Ok(())
        }

        budgey_cli::Commands::Commit { message } => {
            update_pile_with_action(&context, |current_pile| {
                if current_pile.current_staged_transactions.is_empty() {
                    println!("No staged transactions to commit. Add some transactions first.");
                    return Ok(current_pile);
                }
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                    .as_secs()
                    .to_string();
                let balance = current_pile.current_balance;
                trace!("Difference: {}", balance);

                let new_record = &Record::new(
                    &message,
                    &current_time,
                    balance,
                    &current_pile.current_staged_transactions,
                );

                let new_pile = current_pile
                    .add_record(new_record)
                    .clear_staged_transactions();

                println!(
                    "Record {} committed. Balance: {}",
                    new_record.id, new_record.amount_after_record
                );
                Ok(new_pile)
            })?;
            Ok(())
        }
        budgey_cli::Commands::Withdraw { amount } => {
            trace!("Withdrawing from pile: amount: {:?}", amount);
            let new_pile = update_pile_with_action(&context, |pile| {
                Ok(
                    pile.add_transaction(&models::record_transaction::Transaction::new(
                        TransactionType::Withdraw,
                        amount,
                    )),
                )
            })?;

            println!(
                "Staged transaction of {}. Pile now at: {}",
                amount, new_pile.current_balance
            );
            Ok(())
        }
        budgey_cli::Commands::Restore => todo!(),
    }
}

fn update_pile_with_action(
    context: &BudgeyContext,
    action: impl Fn(models::pile::Pile) -> anyhow::Result<models::pile::Pile>,
) -> anyhow::Result<models::pile::Pile> {
    let current_pile = pile_management::get_current_pile(context)?;
    let new_pile = action(current_pile)?;
    pile_management::update_pile(context, &new_pile)?;
    Ok(new_pile)
}
