use super::QueenState;
use Queens;

pub fn get_queen_states(queens: &Queens) -> Vec<QueenState> {
    queens
        .get_queen_positions()
        .iter()
        .map(|&pos| QueenState {
            pos: pos,
            moves: queens.get_queen_moves(pos),
        })
        .collect()
}
