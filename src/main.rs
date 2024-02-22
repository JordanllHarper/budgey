use clap::Parser;

mod budgeter_cli;
pub mod config;
pub mod models;
pub mod repo;
fn main() {
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
}
