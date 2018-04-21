use position_types::*;
use {PosError, Square};

mod board_from_pos_iter;
mod board_queens;
mod board_reflect;
mod board_to_string;

#[derive(Clone, Debug)]
pub struct Board {
    width: u32,
    height: u32,
    queens: CoordSet,
}

impl Board {
    /// Create a new, empty, 8x8 chess board.
    pub fn new() -> Board {
        let width = 8;
        let height = 8;
        let queens = CoordSet::new();
        Board {
            width,
            height,
            queens,
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

    /// Return a pair of coordinates representing the dimensions of the board.
    pub fn dims(&self) -> PosCoords {
        (self.width, self.height)
    }

    /// Return a bool representing whether or not a position is in bounds.
    pub fn in_bounds(&self, pos: &PosCoords) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    /// Return the contents of a square. Returns an error if the position is
    /// not within the bounds of the board.
    pub fn get_square(&self, row: u32, col: u32) -> Result<Square, PosError> {
        let pos = (col, row);
        if !self.in_bounds(&pos) {
            return Err(PosError::OutOfBounds);
        } else {
            match self.queens.contains(&pos) {
                true => Ok(Square::Queen),
                false => Ok(Square::Empty),
            }
        }
    }

    /// Add a queen to the board at the given position.
    pub fn add_queen(&mut self, pos: PosCoords) {
        if self.in_bounds(&pos) {
            self.queens.insert(pos);
        } else {
            panic!("Cannot add queen at position {:?}", pos);
        }
    }

    /// Get a clone of the hash set containing the queen's positions.
    pub fn get_queen_positions(&self) -> CoordSet {
        self.queens.clone()
    }
}

#[cfg(test)]
mod board_tests {
    use super::Board;
    use position_types::*;
    use {Queens, Square};

    /// Test that the dimensions of a default board are correct.
    #[test]
    fn dimensions_are_correct() {
        let b = Board::new();
        assert_eq!(b.width(), 8);
        assert_eq!(b.height(), 8);
        let dims = b.dims();
        let expected_dims = (8, 8);
        assert_eq!(dims, expected_dims);
    }

    /// Check that the `get_square` method works, and returns a Square::Queen
    /// after adding a queen to that square.
    #[test]
    fn get_square_works() {
        let mut b = Board::new();
        let (x, y) = (0, 0);
        let mut s = b.get_square(x, y);
        assert_eq!(s, Ok(Square::Empty));
        b.add_queen(x, y);
        s = b.get_square(x, y);
        assert_eq!(s, Ok(Square::Queen));
    }

    /// Test that `get_square` will fail gracefully given a coordinate pair
    /// that is out of bounds of the board.
    #[test]
    fn get_square_handles_oob_coords() {
        let b = Board::new();
        let (x, y) = (8, 8);
        let s = b.get_square(x, y);
        assert_eq!(s, Err(PosError::OutOfBounds));
    }

    /// Helper struct used to test the `get_pos_index` and `get_index_pos`
    /// helper functions. Each struct should contain corresponding index
    /// coordinate pairs.
    struct PosTestCase {
        coords: PosCoords,
        i: PosIndex,
    }

    /// Test cases.
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
