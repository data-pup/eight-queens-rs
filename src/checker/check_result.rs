/// Represents a given solution state.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckResult {
    pub is_solved: bool,
    pub has_conflict: bool,
    pub num_queens: u8,
    pub num_free_spaces: u8,
}
