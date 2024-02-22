use clap::Parser;

pub mod cli;
pub mod config;
pub mod models;
pub mod repo;
fn main() {
    let commands = budgeter_cli::BudgeyCLI::parse().commands;
    match commands {
        budgeter_cli::Commands::Init { name } => todo!(),
        budgeter_cli::Commands::Pile { subcommand } => todo!(),
    }
}
