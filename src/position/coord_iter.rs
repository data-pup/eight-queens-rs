use PosCoords;

pub struct CoordIter {
    height: u32,
    width: u32,
    curr_i: u32,
}

impl CoordIter {
    /// Returns true if the iterator is still in bounds.
    fn in_bounds(&self) -> bool {
        self.curr_i < self.width * self.height
    }

    /// Returns the current position coordinates.
    fn curr_pos(&self) -> PosCoords {
        let y = self.curr_i / self.width;
        let x = self.curr_i % self.width;
        (x, y)
    }
}

// Create a coordinate space iterator using given dimensions.
impl From<PosCoords> for CoordIter {
    fn from(dims: PosCoords) -> CoordIter {
        let (width, height) = dims;
        CoordIter {
            height,
            width,
            curr_i: 0,
        }
    }
}

// Implement the iterator trait for the coordinate iterator object.
impl Iterator for CoordIter {
    type Item = PosCoords;
    fn next(&mut self) -> Option<Self::Item> {
        match self.in_bounds() {
            true => {
                let res = self.curr_pos();
                self.curr_i += 1;
                Some(res)
            }
            false => None,
        }
    }
}

#[cfg(test)]
mod coord_iter_tests {
    use super::CoordIter;
    use PosCoords;

    #[test]
    fn coord_iter_can_be_created_from_dims() {
        let coord_iter = CoordIter::from((8, 8));
        assert_eq!(coord_iter.height, 8);
        assert_eq!(coord_iter.width, 8);
        assert_eq!(coord_iter.curr_i, 0);
    }

    #[test]
    fn iterating_3_by_3_space_works() {
        let dims = (3, 3);
        let coord_iter = CoordIter::from(dims);
        let coords = coord_iter.collect::<Vec<PosCoords>>();
        let expected = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];
        assert_eq!(coords, expected);
    }
}

#[cfg(test)]
mod coord_iter_benches {
    extern crate test;

    use self::test::Bencher;
    use super::CoordIter;
    use PosCoords;

    #[bench]
    fn time_collection_of_3_by_3_coord_space(bencher: &mut Bencher) {
        let (width, height) = (3, 3);
        bencher.iter(|| collect_coord_space(width, height));
    }

    #[bench]
    fn time_collection_of_8_by_8_coord_space(bencher: &mut Bencher) {
        let (width, height) = (8, 8);
        bencher.iter(|| collect_coord_space(width, height));
    }

    fn collect_coord_space(width: u32, height: u32) -> Vec<PosCoords> {
        let dims = (width, height);
        let coord_iter = CoordIter::from(dims);
        coord_iter.collect()
    }
}
