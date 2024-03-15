use clap::{Parser, Subcommand};

/// A finance tracking and budgeting tool
#[derive(Debug, Parser)]
pub struct BudgeyCLI {
    #[command(subcommand)]
    pub commands: Commands,
}

/// Commands for the Budgey CLI
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Budgey init -> inits a new budget to work with and creates a default pile
    /// called "main".
    #[command(name = "init", arg_required_else_help = true)]
    Init {
        /// The name of the new budget. Must be unique.
        #[arg()]
        name: String,
    },
    Budget {
        #[command(subcommand)]
        subcommand: BudgetSubcommand,
    },
    /// Budgey Pile -> create and manage piles.
    #[command(name = "pile")]
    Pile {
        ///The subcommand to run.
        #[command(subcommand)]
        subcommand: PileSubcommand,
    },
}
#[derive(Debug, Subcommand)]
pub enum BudgetSubcommand {
    /// Set the named working budget.
    #[command(name = "new", arg_required_else_help = true)]
    Focus {
        /// The name of the budget to focus on.
        #[arg(short, long, required = true)]
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum PileSubcommand {
    /// new -> creates a new pile.
    #[command(name = "new", arg_required_else_help = true)]
    New {
        /// The name of the source pile to create the new pile from.
        #[arg(short, long)]
        source: String,

        /// The name of the new pile. Must be unique.
        #[arg(short, long)]
        name: String,

        /// The initial balance of the pile. Will be taken from the source pile.
        #[arg(short, long)]
        initial_balance: Option<f32>,
    },
    /// Add -> Adds a new transaction to the pile.
    #[command(name = "push", arg_required_else_help = true)]
    Add {
        #[arg(short, long, required = true)]
        /// The amount of the transaction.
        amount: f32,

        /// The optional source of the transaction.
        source: Option<String>,
    },

    // Commit -> commits this transaction, making it a part of the transaction history.
    // This can only be undone with a revert.
    #[command(name = "commit", arg_required_else_help = true)]
    Commit {
        /// An optional message for the transaction commit.
        message: Option<String>,
    },

    /// Revert -> reverts a transaction commit.
    #[command(name = "revert", arg_required_else_help = true)]
    Revert {
        /// An optional message for the transaction revert.
        message: Option<String>,
    },

    /// Restore -> restores all added transactions to the last commit.
    #[command(name = "commit", arg_required_else_help = true)]
    Restore,

    /// Merge -> merge a source pile into a destination pile.
    #[command(name = "merge", arg_required_else_help = true)]
    Merge {
        /// The amount of the transaction.
        #[arg(short, long, required = true)]
        amount: f32,

        /// The source pile name.
        #[arg(short, long, required = true)]
        source: String,

        /// The destination pile name.
        #[arg(short, long, required = true)]
        destination: String,

        /// Delete source pile after merge.
        #[arg(short, long)]
        delete_after_merge: bool,

        /// An optional comment for the merge.
        #[arg(short, long)]
        usage: Option<String>,
    },

    /// Balance -> check the balance of a pile.
    #[command(name = "balance", arg_required_else_help = true)]
    Balance {
        /// The name of the pile to check the balance of.
        #[arg(short, long, required = true)]
        name: String,
    },

    /// List -> list available piles.
    #[command(name = "list", arg_required_else_help = true)]
    List,

    /// View -> view transactions of a pile
    #[command(name = "view", arg_required_else_help = true)]
    View {
        /// The name of the pile to view transactions of.
        #[arg(short, long, required = true)]
        name: String,
    },

    /// Remove -> remove a pile.
    #[command(name = "remove", arg_required_else_help = true)]
    Remove {
        /// The name of the pile to remove.
        /// NOTE: use merge if you want to transfer the balance of the pile to another pile with
        /// --delete-after-merge.
        #[arg(short, long, required = true)]
        name: String,
    },
}
