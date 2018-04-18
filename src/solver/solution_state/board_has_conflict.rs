use Queens;

/// This helper method will check whether the board has any contested
/// queens, that could move to one another's space.
pub fn has_contested_queens(b: &Queens) -> bool {
    let q_positions = b.get_queen_positions();
    let uncontested = b.get_uncontested_spaces();
    match q_positions.is_empty() {
        true => false,
        false => q_positions
            .iter()
            .map(|p| uncontested.contains(p))
            .fold(true, |result, is_uncontested| result && is_uncontested),
    }
}
