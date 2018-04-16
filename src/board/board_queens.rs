use std::collections::HashSet;
use {Square, Queens};
use position_types::*;
use super::Board;

impl Queens for Board {
    /// Add a queen to the board at the given position.
    fn add_queen(&mut self, row: u32, col: u32) {
        let i = self.get_pos_index(row, col);
        self.squares[i] = Square::Queen;
    }

    /// Get the positions of the queens.
    fn get_queen_positions(&self) -> HashSet<PosCoords> {
        self.squares.iter().enumerate()
            .filter(|(_, &s)| s == Square::Queen)
            .map(|(i, _)| self.get_index_pos(i))
            .collect::<HashSet<PosCoords>>()
    }

    fn get_queen_moves(&self, pos: PosCoords) -> HashSet<PosCoords> {
        unimplemented!();
    }
}

#[cfg(test)]
mod board_queens_tests {
    use std::collections::HashSet;
    use {Square, Queens};
    use position_types::*;
    use super::Board;

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
}
