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
    /// Initialises Budgey, or make a new budget if budgey is already initialised.
    #[command(name = "init", arg_required_else_help = true)]
    Init {
        /// The name of the new budget. Must be unique.
        name: String,
    },

    /// Create, manage and switch between budgets.
    Budget {
        #[command(subcommand)]
        subcommand: Option<BudgetSubcommand>,
    },

    /// Create, manage and switch piles in the currently focused budget.
    #[command(name = "pile")]
    Pile {
        ///The subcommand to run.
        ///If not provided, will display the current pile.
        #[command(subcommand)]
        subcommand: Option<PileSubcommand>,
    },
}
#[derive(Debug, Subcommand, Clone)]
pub enum BudgetSubcommand {
    /// Set the named working budget.
    #[command(name = "focus", arg_required_else_help = true)]
    Focus {
        /// The name of the budget to focus on.
        name: String,
    },

    /// List all available budgets.
    #[command(name = "ls")]
    List,

    /// Create a new budget.
    #[command(name = "new", arg_required_else_help = true)]
    New { name: String },

    /// Delete a budget.
    #[command(name = "delete", arg_required_else_help = true)]
    Delete { name: String },
}

#[derive(Debug, Subcommand, Clone)]
pub enum PileSubcommand {
    /// Creates a new pile.
    #[command(name = "new", arg_required_else_help = true)]
    New {
        /// The name of the new pile. Must be unique.
        new_pile_name: String,

        /// The name of the source pile to create the new pile from.
        /// If not provided, the new pile will be created from the currently focused pile.
        #[arg(short, long)]
        source: Option<String>,

        /// The initial balance of the pile. Will be 0.0 if not provided.
        #[arg(name = "initial", short, long)]
        initial_balance: Option<f32>,
    },

    /// List available piles.
    #[command(name = "ls")]
    List,

    /// Remove a pile.
    #[command(name = "delete", arg_required_else_help = true)]
    Delete {
        /// The name of the pile to remove.
        /// NOTE: use merge if you want to transfer the balance of the pile to another pile with
        /// --delete-after-merge.
        name: String,
    },

    /// Check the balance of a pile.
    #[command(name = "balance")]
    Balance {
        /// The name of the pile to check the balance of.
        name: Option<String>,
    },

    /// Create an 'add' transaction, where money is added to a pile.
    #[command(name = "add", arg_required_else_help = true)]
    Add {
        /// The amount of the transaction.
        amount: f32,
    },
    },
    /// Focus a new pile.
    #[command(name = "focus", arg_required_else_help = true)]
    Focus {
        /// The name of the pile to focus on.
        name: String,
    },

    // TODO: Mvp stuff
    // Commit -> commits this transaction, making it a part of the transaction history.
    // This can only be undone with a revert.
    #[command(name = "commit", arg_required_else_help = true)]
    Commit {
        /// A required message for the transaction commit.
        ///
        /// This message should be used to describe the transactions made.
        message: String,
    },
    // TODO: Non mvp stuff
    //
    // /// Revert -> reverts a transaction commit.
    // #[command(name = "revert", arg_required_else_help = true)]
    // Revert {
    //     /// An optional message for the transaction revert.
    //     message: Option<String>,
    // },
    //
    // /// Restore -> restores all added transactions to the last commit.
    // #[command(name = "restore", arg_required_else_help = true)]
    // Restore,
    //
    // /// Merge -> merge a source pile into a destination pile.
    // #[command(name = "merge", arg_required_else_help = true)]
    // Merge {
    //     /// The amount of the transaction.
    //     #[arg(short, long, required = true)]
    //     amount: f32,
    //
    //     /// The source pile name.
    //     #[arg(short, long, required = true)]
    //     source: String,
    //
    //     /// The destination pile name.
    //     #[arg(short, long, required = true)]
    //     destination: String,
    //
    //     /// Delete source pile after merge.
    //     #[arg(short, long)]
    //     delete_after_merge: bool,
    //
    //     /// An optional comment for the merge.
    //     #[arg(short, long)]
    //     usage: Option<String>,
    // },
    //

    // /// View -> view transactions of a pile
    // #[command(name = "view", arg_required_else_help = true)]
    // View {
    //     /// The name of the pile to view transactions of.
    //     #[arg(short, long, required = true)]
    //     name: String,
    // },
    //
}
