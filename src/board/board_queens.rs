use super::Board;
use position_types::*;
use std::collections::HashSet;
use {Queens, Square};

impl Queens for Board {
    /// Add a queen to the board at the given position.
    fn add_queen(&mut self, row: u32, col: u32) {
        let i = self.get_pos_index(row, col);
        self.squares[i] = Square::Queen;
    }

    /// Get the positions of the queens.
    fn get_queen_positions(&self) -> HashSet<PosCoords> {
        self.squares
            .iter()
            .enumerate()
            .filter(|(_, &s)| s == Square::Queen)
            .map(|(i, _)| self.get_index_pos(i))
            .collect::<HashSet<PosCoords>>()
    }

    fn get_queen_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        let moves = [
            &self.get_n_moves(pos),
            &self.get_s_moves(pos),
            &self.get_w_moves(pos),
            &self.get_e_moves(pos),
        ];
        moves
            .iter()
            .fold(HashSet::new(), |res, dir| res.union(dir).cloned().collect())
    }
}

// This block stores helper functions for finding the moves in a given
// direction. These functions each return a hash set of coordinates.
impl Board {
    fn get_n_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        (pos.1..self.height).map(|y| (pos.0, y)).collect()
    }

    fn get_s_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        (0..pos.1 + 1).map(|y| (pos.0, y)).collect()
    }

    fn get_w_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        (0..pos.0 + 1).map(|x| (x, pos.1)).collect()
    }

    fn get_e_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        (pos.0..self.width).map(|x| (x, pos.1)).collect()
    }
}

#[cfg(test)]
mod board_queens_tests {
    use super::Board;
    use position_types::*;
    use std::collections::HashSet;
    use {Queens, Square};

    #[test]
    fn add_queen_works() {
        let mut b = Board::new();
        for curr in [(0, 0), (0, 7), (7, 0), (7, 7)].iter() {
            let &(col, row) = curr;
            b.add_queen(row, col);
        }
        let queen_positions = b.get_queen_positions();
        let expected: HashSet<PosCoords> =
            [(0, 0), (0, 7), (7, 0), (7, 7)].iter().cloned().collect();
        assert_eq!(queen_positions, expected);
    }

    #[test]
    fn get_n_moves_works_from_origin() {
        let mut b = Board::new();
        let pos = (0, 0);
        let expected = [
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
        ].iter()
            .cloned()
            .collect();
        let result = b.get_n_moves(pos);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_n_moves_returns_curr_pos_from_top_row() {
        let mut b = Board::new();
        let pos = (0, 7);
        let expected = [(0, 7)].iter().cloned().collect();
        let result = b.get_n_moves(pos);
        assert_eq!(result, expected);
    }
}
