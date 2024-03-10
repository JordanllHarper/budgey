use clap::{Parser, Subcommand};

/// A finance tracking and budgeting tool
#[derive(Debug, Parser)]
#[command()]
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
    /// Budgey Pile -> create and manage piles.
    #[command(name = "pile")]
    Pile {
        ///
        #[command(subcommand)]
        subcommand: PileSubcommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum PileSubcommand {
    /// new -> creates a new pile.
    #[command(name = "new", arg_required_else_help = true)]
    New {
        /// The budget to create the new pile in.
        #[arg(short, long)]
        budget_name: String,

        /// The name of the new pile. Must be unique.
        #[arg(short, long)]
        name: String,

        /// The initial balance of the pile. Will be taken from the main pile.
        #[arg(short, long)]
        initial_balance: Option<f64>,
    },

    #[command(name = "op", arg_required_else_help = true)]
    PileOperation {
        /// The budget to operate on.
        #[arg(short, long)]
        budget_name: String,

        #[command(subcommand)]
        subcommand: PileOperationSubcommand,
    },
}
#[derive(Debug, Subcommand)]
pub enum PileOperationSubcommand {
    /// Add -> Adds a new transaction to the pile.
    #[command(name = "push", arg_required_else_help = true)]
    Add {
        #[arg(short, long, required = true)]
        /// The amount of the transaction.
        amount: f64,

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
    /// Pull -> pulls money from the pile.
    #[command(name = "pull", arg_required_else_help = true)]
    Pull {
        #[arg(short, long, required = true)]
        /// The amount of the transaction.
        amount: f64,

        /// The optional usage of the transaction.
        usage: Option<String>,
    },

    /// Merge -> transfer money from one pile to another.
    #[command(name = "merge", arg_required_else_help = true)]
    Merge {
        /// The amount of the transaction.
        #[arg(short, long, required = true)]
        amount: f64,

        /// The source pile of the transaction.
        #[arg(short, long, required = true)]
        source: String,

        #[arg(short, long, required = true)]
        destination: String,

        /// Delete source pile after merge.
        #[arg(short, long)]
        delete_after_merge: bool,

        /// The optional reason for the merge.
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
