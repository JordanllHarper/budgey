use crate::{
    budgey_config::BudgeyConfig,
    budgey_context::BudgeyContext,
    models::record_transaction::{Record, Transaction, TransactionType},
    utils::round_to_two_decimals,
};
use anyhow::anyhow;
use budgey_cli::Commands;
use clap::Parser;
use colored::Colorize;
use log::{info, trace};
use utils::{concat_paths, create_json_file_name};

mod budget_management;
mod budgey_cli;
mod budgey_config;
mod budgey_context;
mod budgey_state;
mod handle_budget;
mod handle_init;
mod handle_pile;
mod models;
mod pile_management;
mod utils;

fn main() -> anyhow::Result<()> {
    let home = env!("HOME").to_string();
    info!("Home environment initialised: {}", home);

    let budgey_path = format!("{}{}", home, "/.budgey");
    info!("Budgey path: {}", budgey_path);

    let budgey_state_json_name = create_json_file_name("budgey_state");
    let budgey_state_path = concat_paths(&budgey_path, &budgey_state_json_name);
    info!("Budgey state path created: {}", budgey_state_path);

    let budgey_config = BudgeyConfig::new(&budgey_path, &budgey_state_json_name);
    info!("Budgey config created:\n {:#?}", budgey_config);

    info!("Parsing CLI arguments");
    let args = budgey_cli::BudgeyCLI::parse();
    trace!("Parsed CLI arguments: {:#?}", args);

    match args {
        budgey_cli::BudgeyCLI::Init { name } => handle_init::handle_init(budgey_config, &name),
        budgey_cli::BudgeyCLI::Subcommands(c) => {
            let context = BudgeyContext::new(
                &budgey_state::get_budgey_state(&budgey_state_path)?,
                &budgey_config,
            );

            handle_subcommands(&context, c)
        }
    }
}

fn handle_subcommands(context: &BudgeyContext, command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Budget { subcommand } => {
            if let Some(sub) = &subcommand {
                handle_budget::handle_budget_subcommand(context, sub.clone())
            } else {
                let current_budget = budget_management::get_current_budget_name(context)?;
                println!("Current budget: {:?}", current_budget);
                Ok(())
            }
        }
        Commands::Pile { subcommand } => {
            if let Some(sub) = subcommand {
                handle_pile::handle_pile_subcommand(context, sub)?;
            } else {
                let current_pile = pile_management::get_current_pile(context)?;
                println!("Current pile: {}", current_pile.get_name());
            }
            Ok(())
        }
        Commands::Add { amount, note } => {
            trace!("Adding to pile: amount: {:?}", amount);
            let amount = match evalexpr::eval(&amount) {
                Ok(v) => v.as_number()?,
                Err(e) => {
                    println!("Invalid amount or expression. Please try again.");
                    return Err(anyhow!("Invalid amount or expression: {:?}", e));
                }
            };
            let new_pile = update_pile_with_action(context, |pile| {
                Ok(pile.add_transaction(&Transaction::new(
                    TransactionType::Add,
                    round_to_two_decimals(amount as f32),
                    note.as_deref(),
                )))
            })?;

            println!(
                "Staged transaction of {}. Pile now at: {}",
                amount, new_pile.current_balance
            );
            Ok(())
        }

        Commands::Commit { message } => {
            update_pile_with_action(context, |current_pile| {
                if current_pile.current_staged_transactions.is_empty() {
                    println!("No staged transactions to commit. Add some transactions first.");
                    return Ok(current_pile);
                }
                let current_time = utils::get_current_timestamp()?;
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
        Commands::Withdraw { amount, note } => {
            let amount = match evalexpr::eval(&amount) {
                Ok(v) => v.as_number()?,
                Err(e) => {
                    println!("Invalid amount or expression. Please try again.");
                    return Err(anyhow!("Invalid amount or expression: {:?}", e));
                }
            };
            trace!("Withdrawing from pile: amount: {:?}", amount);
            let new_pile = update_pile_with_action(context, |pile| {
                Ok(
                    pile.add_transaction(&models::record_transaction::Transaction::new(
                        TransactionType::Withdraw,
                        round_to_two_decimals(amount as f32),
                        note.as_deref(),
                    )),
                )
            })?;

            println!(
                "Staged transaction of {}. Pile now at: {}",
                amount, new_pile.current_balance
            );
            Ok(())
        }
        Commands::Restore => {
            let updated_pile = update_pile_with_action(context, |pile| {
                let new_balance = pile
                    .records
                    .last()
                    .map(|record| record.amount_after_record)
                    .unwrap_or_else(|| pile.current_balance);
                let new_pile = pile.set_balance(new_balance);

                Ok(new_pile.clear_staged_transactions())
            })?;

            println!(
                "Restored to last record. Pile now at: {}",
                updated_pile.current_balance
            );

            Ok(())
        }
        Commands::Log => {
            let current_pile = pile_management::get_current_pile(context)?;

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
            let current_pile = pile_management::get_current_pile(context)?;
            handle_showing_transactions(&current_pile)?;
            Ok(())
        }
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
