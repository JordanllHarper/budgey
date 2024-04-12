use crate::{
    budget_management, budgey_cli, budgey_state,
    models::{self, pile::Pile},
    pile_management, BudgeyContext,
};

pub fn handle_budget_subcommand(
    context: &BudgeyContext,
    subcommand: budgey_cli::BudgetSubcommand,
) -> anyhow::Result<()> {
    match subcommand {
        budgey_cli::BudgetSubcommand::Focus { name } => {
            let state = &context.state;
            let budget_exists = budget_management::does_budget_exist(&context, &name)?;

            if !budget_exists {
                println!("Budget doesn't exist, specify another name");
                return Ok(());
            }

            let new_state = state.change_focused_budget_name(&name);
            budgey_state::write_budgey_state(&context.budgey_config, &new_state)?;
            println!("Checked out new budget: {}", name);
            Ok(())
        }
        budgey_cli::BudgetSubcommand::New { name } => {
            let budget_exists = budget_management::does_budget_exist(&context, &name)?;
            if budget_exists {
                println!("Budget already exists with the same name");
                return Ok(());
            }
            budget_management::create_new_budget(
                &context.budgey_config.get_budget_path(&name),
                models::budget::Budget::new_init(&name),
            )?;
            pile_management::create_new_pile(&context, &Pile::default_main_pile())?;
            let new_state = context
                .state
                .add_budget_name(&name)
                .change_focused_budget_name(&name);
            budgey_state::write_budgey_state(&context.budgey_config, &new_state)?;
            println!("Created and focused new budget: {}", name);
            Ok(())
        }
        budgey_cli::BudgetSubcommand::Delete { name } => {
            let budget_exists = budget_management::does_budget_exist(&context, &name)?;
            if !budget_exists {
                println!("Budget doesn't exist, specify another name");
                return Ok(());
            }

            budget_management::delete_budget(context, &name)?;
            println!("Deleted budget: {}", name);
            Ok(())
        }
        budgey_cli::BudgetSubcommand::List => {
            let budget_names = &context.state.budget_names;

            if budget_names.is_empty() {
                println!("No budgets found. Type budgey budget new <name> to create a new budget.");
                return Ok(());
            };
            println!("Budgets: ");

            for name in budget_names {
                if name == &context.state.current_focused_budget_name {
                    println!(" * {}", name);
                } else {
                    println!(" - {}", name);
                }
            }

            Ok(())
        }
    }
}
