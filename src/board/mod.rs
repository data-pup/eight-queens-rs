use position_types::*;
use Square;

mod board_from_pos_iter;
mod board_queens;
mod board_to_string;
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
        let squares = (0..width * height).map(|_| Square::Empty).collect();
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

    /// Return the contents of a square.
    pub fn get_square(&self, row: u32, col: u32) -> Square {
        let i = self.get_pos_index(row, col);
        self.squares[i]
    }
}

impl Board {
    /// Find the index for a given position's coordinates.
    fn get_pos_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Find the position coordinates for a given index.
    fn get_index_pos(&self, pos: usize) -> PosCoords {
        let y = pos as u32 / self.width;
        let x = pos as u32 % self.width;
        (x, y)
    }
}

#[cfg(test)]
mod board_tests {
    use super::Board;
    use position_types::*;

    #[test]
    fn dimensions_are_correct() {
        let b = Board::new();
        assert_eq!(b.width(), 8);
        assert_eq!(b.height(), 8);
    }

    #[test]
    fn get_pos_index_works() {
        let b = Board::new();
        for PosTestCase {
            coords: (col, row),
            i: expected,
        } in POS_TEST_CASES.iter()
        {
            let result = b.get_pos_index(*row, *col);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn get_index_pos_works() {
        let b = Board::new();
        for PosTestCase {
            coords: expected,
            i: pos,
        } in POS_TEST_CASES.iter()
        {
            let result = b.get_index_pos(*pos);
            assert_eq!(result, *expected);
        }
    }

    struct PosTestCase {
        coords: PosCoords,
        i: PosIndex,
    }

    static POS_TEST_CASES: &[PosTestCase] = &[
        PosTestCase {
            coords: (0, 0),
            i: 0_usize,
        },
        PosTestCase {
            coords: (7, 0),
            i: 7_usize,
        },
        PosTestCase {
            coords: (0, 1),
            i: 8_usize,
        },
        PosTestCase {
            coords: (0, 7),
            i: 56_usize,
        },
        PosTestCase {
            coords: (7, 7),
            i: 63_usize,
        },
    ];
}
