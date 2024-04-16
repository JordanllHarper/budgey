use log::trace;

use crate::{
    budget_management::{get_current_budget, update_budget},
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
            let pile = match maybe_get_user_defined_pile(&context, source.as_deref())? {
                Some(source_pile) => source_pile,
                None => {
                    println!("Couldn't get the source pile specified");
                    return Ok(());
                }
            };
            let initial_balance = pile.current_balance;

            let new_pile = Pile::new_user_created(initial_balance, &new_pile_name, &pile.records);
            create_new_pile(&context, &new_pile)?;
            let budget = get_current_budget(&context)?
                .add_pile(&new_pile_name)
                .change_current_pile(&new_pile_name);
            update_budget(&context.get_current_budget_path(), budget)?;

            Ok(())
        }
        budgey_cli::PileSubcommand::List => {
            let current_budget = get_current_budget(&context)?;
            let pile_names = &current_budget.pile_names;

            if pile_names.is_empty() {
                println!("No piles found. \n\nType `budgey pile new <name>` to create a new pile");
                return Ok(());
            };
            println!("Piles: ");

            for name in pile_names {
                if name == &current_budget.current_pile_name {
                    println!(" * {}", name);
                } else {
                    println!(" | {}", name);
                }
            }

            Ok(())
        }
        budgey_cli::PileSubcommand::Delete { name } => {
            if name.to_lowercase() == "main" {
                println!("Cannot delete the main pile of a budget.\n\nIf you want to delete the budget, type `budget delete <name>`");
                return Ok(());
            }
            pile_management::delete_pile(&context, &name)?;
            let current_budget = get_current_budget(&context)?.delete_pile(&name);
            update_budget(&context.get_current_budget_path(), current_budget)
        }
        budgey_cli::PileSubcommand::Balance { name } => {
            let get_pile = maybe_get_user_defined_pile(&context, name.as_deref())?;
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
            let current_budget = get_current_budget(&context)?;
            if !current_budget.pile_names.contains(&name) {
                println!("Pile doesn't exist in the current budget. Specify another name.");
                return Ok(());
            }
            let new = current_budget.change_current_pile(&name);
            update_budget(&context.get_current_budget_path(), new)?;
            println!("Focused on pile: {}", name);
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
