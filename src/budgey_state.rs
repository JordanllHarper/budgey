use crate::models::budgey_state::BudgeyState;

pub struct BudgeyConfig {
    budgey_path_to_json: String,
}

pub fn get_budgey_state(budgey_path_to_json: &str) -> anyhow::Result<BudgeyState> {
    // TODO: Look up state
    todo!()
}
