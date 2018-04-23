use {Board, PosCoords};

pub struct CoordIter {
    height: u32,
    width: u32,
    curr_x: u32,
    curr_y: u32,
}

impl CoordIter {
    fn in_bounds(&self) -> bool {
        self.curr_x < self.width && self.curr_y < self.height
    }

    fn curr_pos(&self) -> PosCoords {
        (self.curr_x, self.curr_y)
    }

    fn next_pos(&self) -> PosCoords {
        let next_x = match self.curr_x < self.width - 1 {
            true => self.curr_x + 1,
            false => 0,
        };
        let next_y = match self.curr_y < self.height - 1 {
            true => self.curr_y + 1,
            false => 0,
        };
        (next_x, next_y)
    }
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

impl Iterator for CoordIter {
    type Item = PosCoords;
    fn next(&mut self) -> Option<Self::Item> {
        match self.in_bounds() {
            true => {
                let res = self.curr_pos();
                let (next_x, next_y) = self.next_pos();
                self.curr_x = next_x;
                self.curr_y = next_y;
                Some(res)
            }
            false => None,
        }
    }
}

#[cfg(test)]
mod coord_iter_tests {
    use super::CoordIter;
    use {Board, PosCoords};

    #[test]
    fn coord_iter_can_be_created_from_board() {
        let coord_iter = CoordIter::from(Board::new());
        assert_eq!(coord_iter.height, 8);
        assert_eq!(coord_iter.width, 8);
    }

    #[test]
    fn iterating_3_by_3_space_works() {
        let dims = (3, 3);
        let board = Board::from(dims);
        let coord_iter = CoordIter::from(board);
        let coords = coord_iter.collect::<Vec<PosCoords>>();
        let expected = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        assert_eq!(coords, expected);
    }
}
