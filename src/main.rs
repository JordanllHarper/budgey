use utils::create_json_path;

mod budgey_state;
pub mod models;
pub mod utils;

fn main() {
    let budgey_path = "budget_state";
    let budgey_state_name = "budget_state";
    let budgey_state_path = create_json_path(budgey_path, budgey_state_name);
    let Ok(budgey_state) = budgey_state::get_budgey_state(&budgey_state_path) else {
        println!("Couldn't get the budgey state.");
        return;
    };
    // commands
}
