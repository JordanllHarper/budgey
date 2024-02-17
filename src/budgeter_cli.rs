use clap::{Parser, Subcommand};

/// A finance tracking and budgeting tool
#[derive(Debug, Parser)]
#[command()]
struct BudgeterCLI {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "init")]
    Init {},
}
