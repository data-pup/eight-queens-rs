use std::collections::HashSet;
use {PosCoords, Queens};

type CoordSet = HashSet<PosCoords>;

/// This helper method will check whether the board has any contested
/// queens, that could move to one another's space.
pub fn has_contested_queens(q_positions: &CoordSet, uncontested: &CoordSet) -> bool {
    match q_positions.is_empty() {
        true => false,
        false => q_positions
            .iter()
            .map(|p| uncontested.contains(p))
            .fold(true, |result, is_uncontested| result && is_uncontested),
    }
}
