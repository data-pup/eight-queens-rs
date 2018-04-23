use Board;

struct CoordIter {
    _width: u32,
    _height: u32,
}

impl From<Board> for CoordIter {
    fn from(_board: Board) -> CoordIter {
        unimplemented!();
    }
}

// impl IntoIterator for CoordIter {
// }

#[cfg(test)]
mod coord_iter_tests {
    use super::CoordIter;

    #[test]
    fn coord_iter_can_be_created_from_board() {
        unimplemented!();
    }

    #[test]
    fn iterating_3_by_3_space_works() {
        unimplemented!();
    }
}
