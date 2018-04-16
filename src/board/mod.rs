use Square;
use position_types::*;

mod board_queens;
mod board_uncontested_spaces;

#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
    squares: Vec<Square>,
}

impl Board {
    pub fn new() -> Board {
        let width = 8;
        let height = 8;
        let squares = (0..width * height)
            .map(|_| Square::Empty)
            .collect();
        Board {
            width,
            height,
            squares,
        }
    }

    /// Return the width of the board as a u32 object.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Return the width of the board as a u32 object.
    pub fn height(&self) -> u32 {
        self.height
    }

    // FIXUP: This can be removed after implementing the queen position function.
    pub fn queen_count(&self) -> u32 {
        self.squares.iter().filter(|s| **s == Square::Queen).count() as u32
    }
}

impl Board {
    fn get_pos_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn get_index_pos(&self, pos: usize) -> PosIndex {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn dimensions_are_correct() {
        let b = Board::new();
        assert_eq!(b.width(), 8);
        assert_eq!(b.height(), 8);
    }

    #[test]
    fn get_pos_index_works() {
        let b = Board::new();
        let tests: &[(u32, u32, usize)] = &[
            (0, 0, 0_usize),  // Bottom left corner.
            (0, 7, 7_usize),  // Bottom right corner.
            (7, 0, 56_usize), // Top left corner.
            (7, 7, 63_usize), // Top right corner.
        ];
        for &(row, col, expected) in tests.into_iter() {
            let result = b.get_pos_index(row, col);
            assert_eq!(result, expected);
        }
    }
}