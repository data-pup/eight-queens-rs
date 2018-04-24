use std::cmp::Ordering;

/// Represents a given solution state.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct CheckResult {
    pub is_solved: bool,
    pub has_conflict: bool,
    pub num_queens: u8,
    pub num_free_spaces: u8,
}

impl Ord for CheckResult {
    fn cmp(&self, other: &CheckResult) -> Ordering {
        self.num_free_spaces.cmp(&other.num_free_spaces)
    }
}
