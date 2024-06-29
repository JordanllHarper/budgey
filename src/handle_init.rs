use crate::{
    budgey_state::BudgeyState,
    file::{budget_io::BudgetIO, state_io::StateIO},
    models::{budget::Budget, pile::Pile},
    pile_management, BudgeyContext,
};

pub fn handle_init(
    starting_budget_name: &str,
    context: &BudgeyContext,
    state_io: impl StateIO,
    budget_io: impl BudgetIO,
) -> anyhow::Result<()> {
    let budget_already_initialised = state_io.check_state_initialised()?;

    if budget_already_initialised {
        println!(
            "Budgey already initialised. Run `budgey budget new <budget name>` to create new budgets. "
        );
        return Ok(());
    }
    println!("Initialising Budgey...");

    let new_budget = &Budget::new_init(starting_budget_name);
    budget_io.create_new_budget(new_budget)?;

    let init_state = BudgeyState::new_init(starting_budget_name);
    state_io.write_budgey_state(&init_state)?;

    let new_context = BudgeyContext::new(&init_state, &context.config);
    pile_management::create_new_pile(&new_context, &Pile::default_main_pile())?;

    println!("Budgey init finished. Run `budgey` to see help.");
    Ok(())
}
