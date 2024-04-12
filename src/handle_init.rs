use crate::{
    budget_management, budgey_state,
    models::{budget::Budget, budgey_state::BudgeyState, pile::Pile},
    pile_management, utils, BudgeyConfig, BudgeyContext,
};

pub fn handle_init(budgey_config: BudgeyConfig, starting_budget_name: &str) -> anyhow::Result<()> {
    let budget_already_initialised = budgey_state::check_budgey_state_initialised(&budgey_config)?;

    if budget_already_initialised {
        println!(
            "Budgey already initialised. Run `budgey budget new <budget name>` to create new budgets. "
        );
        return Ok(());
    } else {
        println!("Initialising Budgey...");
        budgey_state::write_budgey_state(
            &budgey_config,
            &BudgeyState::new_init(starting_budget_name),
        )?;
    }
    let budget_path = utils::concat_paths(&budgey_config.budgey_path, starting_budget_name);
    println!("Creating new budget: {}", starting_budget_name);
    budget_management::create_new_budget(&budget_path, Budget::new_init(starting_budget_name))?;

    let context = BudgeyContext::new(&BudgeyState::new_init(starting_budget_name), &budgey_config);
    pile_management::create_new_pile(&context, &Pile::default_main_pile())?;

    println!("Budgey init finished. Run `budgey` to see help.");
    Ok(())
}
