use Board;

struct CoordIter {
    height: u32,
    width: u32,
}

impl From<Board> for CoordIter {
    fn from(board: Board) -> CoordIter {
        let (width, height) = board.dims();
        CoordIter { height, width }
    }
}

// impl IntoIterator for CoordIter {
// }

#[cfg(test)]
mod coord_iter_tests {
    use Board;
    use super::CoordIter;

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
