use colored::Colorize;
use log::trace;

use crate::{
    budget_management::{get_current_budget, read_from_str_path, update_budget},
    budgey_cli,
    models::pile::Pile,
    pile_management::{self, *},
    BudgeyContext,
};

pub fn handle_pile_subcommand(
    context: &BudgeyContext,
    subcommand: budgey_cli::PileSubcommand,
) -> anyhow::Result<()> {
    trace!("Handling pile subcommand: {:?}", subcommand);
    match subcommand {
        budgey_cli::PileSubcommand::New {
            source,
            new_pile_name,
        } => {
            let pile = match maybe_get_user_defined_pile(context, source.as_deref())? {
                Some(source_pile) => source_pile,
                None => {
                    println!("Couldn't get the source pile specified");
                    return Ok(());
                }
            };
            let initial_balance = pile.current_balance;

            let new_pile = Pile::new_user_created(initial_balance, &new_pile_name, &pile.records);
            create_new_pile(context, &new_pile)?;
            let budget = get_current_budget(context, read_from_str_path)?
                .add_pile(&new_pile_name)
                .change_current_pile(&new_pile_name);
            update_budget(&context.get_current_budget_path(), budget)?;

            println!("Created and checked out new pile: {}", new_pile_name);

            Ok(())
        }
        budgey_cli::PileSubcommand::List => {
            let current_budget = get_current_budget(context, read_from_str_path)?;
            let pile_names = &current_budget.pile_names;

            if pile_names.is_empty() {
                println!("No piles found. \n\nType `budgey pile new <name>` to create a new pile");
                return Ok(());
            };
            println!("Piles: ");

            for name in pile_names {
                let sign = if name == &current_budget.current_pile_name {
                    "*".green()
                } else {
                    "|".white()
                };

                println!(" {} {}", sign, name);
            }

            Ok(())
        }
        budgey_cli::PileSubcommand::Delete { name } => {
            if name.to_lowercase() == "main" {
                println!("Cannot delete the main pile of a budget.\n\nIf you want to delete the budget, type `budget delete <name>`");
                return Ok(());
            }
            pile_management::delete_pile(context, &name)?;
            let current_budget =
                get_current_budget(context, read_from_str_path)?.delete_pile(&name);
            update_budget(&context.get_current_budget_path(), current_budget)?;
            println!("Deleted pile: {}", name);
            Ok(())
        }
        budgey_cli::PileSubcommand::Balance { name } => {
            let get_pile = maybe_get_user_defined_pile(context, name.as_deref())?;
            if let Some(pile) = get_pile {
                println!(
                    "Balance of pile {}: {}",
                    pile.get_name(),
                    pile.current_balance
                );
                Ok(())
            } else {
                println!("Couldn't get the pile specified");
                Ok(())
            }
        }

        budgey_cli::PileSubcommand::Focus { name } => {
            let current_budget = get_current_budget(context, read_from_str_path)?;
            if !current_budget.pile_names.contains(&name) {
                println!("Pile doesn't exist in the current budget. Specify another name.");
                return Ok(());
            }
            let new_budget = current_budget.change_current_pile(&name);
            update_budget(&context.get_current_budget_path(), new_budget)?;
            let amount = get_current_pile(context)?.current_balance;
            let no_current_staged_transactions =
                get_current_pile(context)?.current_staged_transactions.len();
            println!("Focused pile: {}\nAmount: {}", name, amount);
            if no_current_staged_transactions > 0 {
                println!("Staged transactions: {}", no_current_staged_transactions);
            } else {
                println!("No staged transactions");
            }
            Ok(())
        }
    }
}

/// Gets the user defined pile if it exists, or the current pile if not.
fn maybe_get_user_defined_pile(
    context: &BudgeyContext,
    maybe_pile_name: Option<&str>,
) -> anyhow::Result<Option<Pile>> {
    if let Some(p) = maybe_pile_name {
        maybe_get_pile(context, p)
    } else {
        Ok(Some(get_current_pile(context)?))
    }
}
