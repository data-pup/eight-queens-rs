use std::convert::From;

use {Board, CoordSet, PosCoords};

impl From<PosCoords> for Board {
    fn from(dims: PosCoords) -> Board {
        let (width, height) = dims;
        Board {
            height,
            width,
            queens: CoordSet::new(),
        }
    }
}

#[cfg(test)]
mod board_from_coords_tests {
    use Board;

    #[test]
    fn board_of_size_3_by_3_can_be_created_from_coords() {
        let dims = (3, 3);
        let board = Board::from(dims);
        assert_eq!(board.dims(), dims);
        [(0, 0), (0, 2), (2, 0), (2, 2)].iter().for_each(|pos| {
            assert_eq!(board.in_bounds(pos), true);
        });
        [(0, 3), (3, 0), (3, 3)].iter().for_each(|pos| {
            assert_eq!(board.in_bounds(pos), false);
        });
    }
}
