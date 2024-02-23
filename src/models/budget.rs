use super::pile::Pile;

/// Represents a collection of piles that a user might have
/// Stored in /budgey/[repo_name]/
pub struct Budget {
    repo_name: String,
    pile_list: Vec<Pile>,
}

impl Budget {
    pub fn new(repo_name: String, pile_list: Vec<Pile>) -> Self {
        Self {
            repo_name,
            pile_list,
        }
    }
}
