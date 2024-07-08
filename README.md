# Budgey - a CLI budgeting tool in the style of Git.

This tool incorporates the ideas of Git version control to help you manage your finances.

It allows for creating various `budgets`, branching off with `piles`, staging expenditures, and creating `transaction`s to record and plan income and outgoings.

# Requirements
- Rust tooling - Cargo, Rustc

# Installation  
The current method of installation is to clone this repository, build the source, and reference the binary in your PATH environment variable.

Ideally, a future improvement would add this to package manager remote repositories.

# Usage
Run `budgey init` to initialise a `.budgey` directory, where the required information will be stored.

Then run budgey --help to see commands. 

### Concepts 

Analogies to Git:
Budget -> Repository
Piles -> Branches
Transaction -> Staged changes
Record -> Commit
