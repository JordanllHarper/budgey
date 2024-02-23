use super::pile::Pile;

/// Represents a collection of piles that a user might have
pub struct Repository {
    repo_name: String,
    pile_list: Vec<Pile>,
}

impl Repository {
    pub fn new(repo_name: String, pile_list: Vec<Pile>) -> Self {
        Self {
            repo_name,
            pile_list,
        }
    }
}
