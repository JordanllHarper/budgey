use colored::Colorize;

use crate::{
    budget_management::update_budget,
    budgey_cli,
    file::{budget_io::BudgetIO, pile_io::PileIO},
    models::pile::Pile,
    BudgeyContext,
};

pub fn handle_pile_subcommand(
    context: &BudgeyContext,
    subcommand: budgey_cli::PileSubcommand,
    budget_io: &impl BudgetIO,
    pile_io: &impl PileIO,
) -> anyhow::Result<()> {
    match subcommand {
        budgey_cli::PileSubcommand::New {
            source,
            new_pile_name,
        } => {
            let pile = match maybe_get_user_defined_pile(
                context,
                source.as_deref(),
                budget_io,
                pile_io,
            )? {
                Some(source_pile) => source_pile,
                None => {
                    println!("Couldn't get the source pile specified");
                    return Ok(());
                }
            };
            let initial_balance = pile.current_balance;

            let new_pile = Pile::new_user_created(initial_balance, &new_pile_name, &pile.records);
            pile_io.create_new_pile(&new_pile)?;
            let budget = budget_io
                .get_current_budget(context)?
                .add_pile(&new_pile_name)
                .change_current_pile(&new_pile_name);
            update_budget(&context.get_current_budget_path(), &budget)?;

            println!("Created and checked out new pile: {}", new_pile_name);

            Ok(())
        }
        budgey_cli::PileSubcommand::List => {
            let current_budget = budget_io.get_current_budget(context)?;
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
            let current_budget = budget_io.get_current_budget(context)?;
            pile_io.delete_pile(&name, &current_budget)?;
            let current_budget = budget_io.get_current_budget(context)?.delete_pile(&name);
            update_budget(&context.get_current_budget_path(), &current_budget)?;
            println!("Deleted pile: {}", name);
            Ok(())
        }
        budgey_cli::PileSubcommand::Balance { name } => {
            let get_pile =
                maybe_get_user_defined_pile(context, name.as_deref(), budget_io, pile_io)?;
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
            let current_budget = budget_io.get_current_budget(context)?;
            if !current_budget.pile_names.contains(&name) {
                println!("Pile doesn't exist in the current budget. Specify another name.");
                return Ok(());
            }
            let new_budget = current_budget.change_current_pile(&name);
            update_budget(&context.get_current_budget_path(), &new_budget)?;
            let amount = pile_io.get_current_pile(&new_budget)?.current_balance;
            let no_current_staged_transactions = pile_io
                .get_current_pile(&new_budget)?
                .current_staged_transactions
                .len();
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
    budget_io: &impl BudgetIO,
    pile_io: &impl PileIO,
) -> anyhow::Result<Option<Pile>> {
    let current_budget = budget_io.get_current_budget(context)?;
    if let Some(p) = maybe_pile_name {
        pile_io.maybe_get_pile(p, &current_budget)
    } else {
        Ok(Some(pile_io.get_current_pile(&current_budget)?))
    }
}
