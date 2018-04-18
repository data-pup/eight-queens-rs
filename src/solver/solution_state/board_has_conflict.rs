use super::QueenState;
use std::collections::HashSet;
use PosCoords;

/// This helper method will check whether the board has any contested
/// queens, that could move to one another's space.
pub fn has_contested_queens(queen_states: &[QueenState]) -> bool {
    queen_states
        .iter()
        .map(|curr_queen| -> bool {
            let other_queens_moves: HashSet<PosCoords> = queen_states
                .iter()
                .filter(|q| curr_queen.pos != q.pos)
                .fold(HashSet::new(), |res, curr| {
                    res.union(&curr.moves).cloned().collect()
                });
            other_queens_moves.contains(&curr_queen.pos)
        })
        .fold(false, |any_contested, curr_is_contested| {
            any_contested || curr_is_contested
        })
}
