use crate::{
    budgey_state::BudgeyState,
    models::record_transaction::{Record, Transaction, TransactionType},
    utils::round_to_two_decimals,
};
use anyhow::anyhow;
use budgey_cli::Commands;
use clap::Parser;
use colored::Colorize;
use file::{
    budget_io::{BudgetIO, BudgetIOImpl},
    pile_io::{PileIO, PileIOImpl},
    state_io::{StateIO, StateIOImpl},
};
use utils::{concat_paths, create_json_file_name};

mod budget_management;
mod budgey_cli;
mod budgey_state;
mod file;
mod handle_budget;
mod handle_init;
mod handle_pile;
mod models;
mod utils;

#[derive(Debug, Clone)]
pub struct BudgeyContext {
    config: BudgeyConfig,
    state: BudgeyState,
}

#[derive(Debug, Clone)]
pub struct BudgeyConfig {
    root_path: String,
    state_json_name: String,
}

impl BudgeyConfig {
    pub fn new(budgey_path: &str, state_json_name: &str) -> Self {
        Self {
            root_path: budgey_path.to_string(),
            state_json_name: state_json_name.to_string(),
        }
    }
    pub fn get_state_path(&self) -> String {
        concat_paths(&self.root_path, &self.state_json_name)
    }
    pub fn get_budget_path(&self, budget_name: &str) -> String {
        concat_paths(&self.root_path, budget_name)
    }
}

impl BudgeyContext {
    pub fn new(state: &BudgeyState, budgey_config: &BudgeyConfig) -> Self {
        Self {
            config: budgey_config.clone(),
            state: state.clone(),
        }
    }
    pub fn get_current_budget_path(&self) -> String {
        let current_budget = &self.state.current_focused_budget_name;
        let budgey_path = &self.config.root_path;
        concat_paths(budgey_path, current_budget)
    }
    pub fn get_current_budget_name(&self) -> String {
        self.state.current_focused_budget_name.clone()
    }
    pub fn get_current_budget_json_path(&self) -> String {
        concat_paths(
            &self.get_current_budget_path(),
            &create_json_file_name(&self.state.current_focused_budget_name),
        )
    }

    pub fn update_state(&self, new_state: &BudgeyState) -> Self {
        Self::new(new_state, &self.config)
    }
    pub fn contains_budget(&self, budget_name: &str) -> bool {
        self.state
            .budget_names
            .iter()
            .any(|name| name == budget_name)
    }
}

fn main() -> anyhow::Result<()> {
    let home = env!("HOME").to_string();

    let root_path = format!("{}{}", home, "/.budgey");

    let state_json_name = create_json_file_name("budgey_state");
    let config = BudgeyConfig::new(&root_path, &state_json_name);
    let args = budgey_cli::BudgeyCLI::parse();

    let state_io = StateIOImpl::new(&config);
    let budget_io = BudgetIOImpl::new(&config);

    match args {
        budgey_cli::BudgeyCLI::Init { name } => {
            handle_init::handle_init(&name, &config, &state_io, &budget_io)
        }
        budgey_cli::BudgeyCLI::Subcommands(c) => {
            let state = state_io.read_budgey_state();
            if let Err(ref e) = state {
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!(
                        "Couldn't find the state file.\n\nPlease run {}",
                        "`budgey init`".green()
                    );
                }
            }

            let context = BudgeyContext::new(&state?, &config);
            let pile_io = PileIOImpl::new(&context);

            handle_subcommands(&context, c, &state_io, &budget_io, &pile_io)
        }
    }
}

fn handle_subcommands(
    context: &BudgeyContext,
    command: Commands,
    state_io: &impl StateIO,
    budget_io: &impl BudgetIO,
    pile_io: &impl PileIO,
) -> anyhow::Result<()> {
    match command {
        Commands::Budget { subcommand } => {
            if let Some(sub) = subcommand {
                handle_budget::handle_budget_subcommand(context, sub, state_io, budget_io, pile_io)
            } else {
                let current_budget = context.get_current_budget_name();
                println!("Current budget: {:?}", current_budget);
                Ok(())
            }
        }
        Commands::Pile { subcommand } => {
            if let Some(sub) = subcommand {
                handle_pile::handle_pile_subcommand(context, sub, budget_io, pile_io)?;
            } else {
                let current_budget = budget_io.get_current_budget(context)?;
                let current_pile = pile_io.get_current_pile(&current_budget)?;
                println!("Current pile: {}", current_pile.get_name());
            }
            Ok(())
        }
        Commands::Add { amount, note } => {
            let amount = match evalexpr::eval(&amount) {
                Ok(v) => v.as_number()?,
                Err(e) => {
                    println!("Invalid amount or expression. Please try again.");
                    return Err(anyhow!("Invalid amount or expression: {:?}", e));
                }
            };
            let new_pile = update_pile_with_action(
                context,
                |pile| {
                    Ok(pile.add_transaction(&Transaction::new(
                        TransactionType::Add,
                        round_to_two_decimals(amount as f32),
                        note.as_deref(),
                    )))
                },
                budget_io,
                pile_io,
            )?;

            println!(
                "Staged transaction of {}. Pile now at: {}",
                amount, new_pile.current_balance
            );
            Ok(())
        }

        Commands::Commit { message } => {
            update_pile_with_action(
                context,
                |current_pile| {
                    if current_pile.current_staged_transactions.is_empty() {
                        println!("No staged transactions to commit. Add some transactions first.");
                        return Ok(current_pile);
                    }
                    let current_time = utils::get_current_timestamp()?;
                    let balance = current_pile.current_balance;

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
                },
                budget_io,
                pile_io,
            )?;
            Ok(())
        }
        Commands::Withdraw { amount, note } => {
            let amount = match evalexpr::eval(&amount) {
                Ok(v) => v.as_number()?,
                Err(e) => {
                    println!("Invalid amount or expression. Please try again.");
                    return Err(anyhow!("Invalid amount or expression: {:?}", e));
                }
            };
            let new_pile = update_pile_with_action(
                context,
                |pile| {
                    Ok(
                        pile.add_transaction(&models::record_transaction::Transaction::new(
                            TransactionType::Withdraw,
                            round_to_two_decimals(amount as f32),
                            note.as_deref(),
                        )),
                    )
                },
                budget_io,
                pile_io,
            )?;

            println!(
                "Staged transaction of {}. Pile now at: {}",
                amount, new_pile.current_balance
            );
            Ok(())
        }
        Commands::Restore => {
            let updated_pile = update_pile_with_action(
                context,
                |pile| {
                    let new_balance = pile
                        .records
                        .last()
                        .map(|record| record.amount_after_record)
                        .unwrap_or_else(|| pile.current_balance);
                    let new_pile = pile.set_balance(new_balance);

                    Ok(new_pile.clear_staged_transactions())
                },
                budget_io,
                pile_io,
            )?;

            println!(
                "Restored to last record. Pile now at: {}",
                updated_pile.current_balance
            );

            Ok(())
        }
        Commands::Log => {
            let current_pile = pile_io.get_current_pile(&budget_io.get_current_budget(context)?)?;

            let records = current_pile.records;
            println!(" --- Current Record ---");
            for record in records.iter().rev() {
                let record_indicator = "*".bold();
                let separators = "|".bold();
                let message = record.message.to_string().yellow();
                let amount_after_record = if record.amount_after_record > 0.0 {
                    format!("+{}", record.amount_after_record).green()
                } else {
                    format!("{}", record.amount_after_record).red()
                };

                println!("{} {} ", record_indicator, record.id.purple());
                println!("{}", separators);
                println!(
                    "{}     Amount after record: {}",
                    separators, amount_after_record
                );
                println!("{}     Message: {}", separators, message);

                println!("{}", separators);
            }
            Ok(())
        }
        Commands::Chain => {
            let current_pile = pile_io.get_current_pile(&budget_io.get_current_budget(context)?)?;
            handle_showing_transactions(&current_pile)?;
            Ok(())
        }
    }
}

fn update_pile_with_action(
    context: &BudgeyContext,
    action: impl Fn(models::pile::Pile) -> anyhow::Result<models::pile::Pile>,
    budget_io: &impl BudgetIO,
    pile_io: &impl PileIO,
) -> anyhow::Result<models::pile::Pile> {
    let current_budget = budget_io.get_current_budget(context)?;
    let current_pile = pile_io.get_current_pile(&current_budget)?;
    let new_pile = action(current_pile)?;
    pile_io.update_pile(&new_pile)?;
    Ok(new_pile)
}
fn handle_showing_transactions(current_pile: &models::pile::Pile) -> anyhow::Result<()> {
    if current_pile.current_staged_transactions.is_empty() {
        println!("No transactions in pile");
        return Ok(());
    }

    println!(" --- End of transaction chain ---");
    for (index, current_transaction) in current_pile
        .current_staged_transactions
        .iter()
        .enumerate()
        .rev()
    {
        let transaction_indicator = "*".bold();
        let separators = "|\n|".bold();
        let sign = match current_transaction.transaction_type {
            TransactionType::Add => "+".green(),
            TransactionType::Withdraw => "-".red(),
            TransactionType::Init => "~".white(),
        };
        let note = if let Some(note) = &current_transaction.note {
            note.to_string()
        } else {
            "".to_string()
        };

        println!(
            "{} {}{}    {}",
            transaction_indicator,
            sign,
            current_transaction.amount,
            note.yellow()
        );
        let start = index == 0;
        if !start {
            println!("{}", separators);
        }
    }
    println!(" --- Start of transaction chain ---");
    Ok(())
}
