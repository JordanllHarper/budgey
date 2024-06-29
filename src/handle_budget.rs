use std::io;

use anyhow::Ok;
use colored::Colorize;

use crate::{
    budget_management, budgey_cli,
    file::{budget_io::BudgetIO, pile_io::PileIO, state_io::StateIO},
    models::{self, pile::Pile},
    BudgeyContext,
};

fn execute_if_budget_exists(
    context: &BudgeyContext,
    name: &str,
    on_exists: impl Fn() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let budget_exists = context.contains_budget(name);

    if !budget_exists {
        println!("Budget doesn't exist, specify another name");
        return Ok(());
    }
    on_exists()
}
pub fn handle_budget_subcommand(
    context: &BudgeyContext,
    subcommand: budgey_cli::BudgetSubcommand,
    state_io: &impl StateIO,
    budget_io: &impl BudgetIO,
    pile_io: &impl PileIO,
) -> anyhow::Result<()> {
    match subcommand {
        budgey_cli::BudgetSubcommand::Focus { name } => {
            execute_if_budget_exists(context, &name, || {
                let new_state = context.state.change_focused_budget_name(&name);
                state_io.write_budgey_state(&new_state)?;
                println!("Checked out new budget: {}", name);
                Ok(())
            })
        }
        budgey_cli::BudgetSubcommand::New { name } => {
            execute_if_budget_exists(context, &name, || {
                budget_io.create_new_budget(&models::budget::Budget::new_init(&name))?;

                let new_state = context
                    .state
                    .add_budget_name(&name)
                    .change_focused_budget_name(&name);

                state_io.write_budgey_state(&new_state)?;

                if let Err(e) = pile_io.create_new_pile(&Pile::default_main_pile()) {
                    if e.kind() == io::ErrorKind::AlreadyExists {
                        println!("Pile \"{}\" already exists, try selecting a different name or deleting the pile", name);
                        return Ok(());
                    } else {
                        return Err(e.into());
                    }
                }
                println!("Created and focused new budget: {}", name);
                Ok(())
            })
        }

        budgey_cli::BudgetSubcommand::Delete { name } => {
            execute_if_budget_exists(context, &name, || {
                budget_management::delete_budget(context, &name)?;
                println!("Deleted budget: {}", name);
                Ok(())
            })
        }
        budgey_cli::BudgetSubcommand::List => {
            let budget_names = &context.state.budget_names;

            if budget_names.is_empty() {
                println!(
                    "No budgets found. \n\nType `budgey budget new <name>` to create a new budget."
                );
                return Ok(());
            };
            println!("Budgets: ");

            for name in budget_names {
                let sign = if name == &context.state.current_focused_budget_name {
                    "*".green()
                } else {
                    "-".white().bold()
                };
                println!(" {} {}", sign, name)
            }

            Ok(())
        }
    }
}
