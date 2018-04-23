use Board;

struct CoordIter {
    height: u32,
    width: u32,
    curr_x: u32,
    curr_y: u32,
}

impl From<Board> for CoordIter {
    fn from(board: Board) -> CoordIter {
        let (width, height) = board.dims();
        CoordIter {
            height,
            width,
            curr_x: 0,
            curr_y: 0,
        }
    }
}

// impl IntoIterator for CoordIter {
// }

#[cfg(test)]
mod coord_iter_tests {
    use super::CoordIter;
    use Board;

    #[test]
    fn coord_iter_can_be_created_from_board() {
        let coord_iter = CoordIter::from(Board::new());
        assert_eq!(coord_iter.height, 8);
        assert_eq!(coord_iter.width, 8);
    }

    #[test]
    fn iterating_3_by_3_space_works() {
        unimplemented!();
    }
}
