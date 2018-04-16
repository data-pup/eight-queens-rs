use Queens;
use position_types::*;
use super::Board;

impl Queens for Board {
    /// Add a queen to the board at the given position.
    fn add_queen(&mut self, row: u32, col: u32) {
        unimplemented!();
    }

    /// Get the positions of the queens.
    fn get_queen_positions(&self) -> Vec<PosCoords> {
        unimplemented!();
    }
}

#[cfg(test)]
mod board_queens_tests {
    // use position_types::*;
    // use super::Board;

    #[test]
    fn it_works() {
        assert_eq!(1, 2, "To do...");
    }
}
