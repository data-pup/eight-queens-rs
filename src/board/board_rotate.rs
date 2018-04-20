use {Board, PosCoords, Rotation};

impl Rotation for Board {
    /// Rotate the queens on the board 90 degrees clockwise.
    fn get_rotate_90_deg_clockwise(&self) -> Board {
        // let positions = self.get_queen_positions();
        unimplemented!();
    }

    /// Rotate the queens on the board 90 degrees counter-clockwise.
    fn get_rotate_90_deg_counter_clockwise(&self) -> Board {
        // let positions = self.get_queen_positions();
        unimplemented!();
    }

    /// Rotate the queens on the board 180 degrees.
    fn get_rotate_180_deg(&self) -> Board {
        // let positions = self.get_queen_positions();
        unimplemented!();
    }
}

impl Board {
    fn rotate_coord_90_deg_clockwise(&self, pos: &PosCoords) -> PosCoords {
        let &(orig_x, orig_y) = pos;
        (
            orig_y,
            (self.height - 1) - orig_x,
        )
    }
}

#[cfg(test)]
mod rotate_tests {
    use {Board, PosCoords, Rotation};

    struct RotateTestCase {
        input: PosCoords,
        expected: PosCoords,
    }

    static TEST_CASES: &[RotateTestCase] = &[
        RotateTestCase {
            input: (0, 7),
            expected: (7, 7),
        },
        RotateTestCase {
            input: (2, 6),
            expected: (5, 6),
        },
        RotateTestCase {
            input: (7, 7),
            expected: (7, 0),
        },
        RotateTestCase {
            input: (5, 6),
            expected: (5, 1),
        },
    ];

    #[test]
    fn rotate_coord_90_deg_clockwise_works() {
        let b = Board::new();
        TEST_CASES.iter().for_each(|test_case| {
            let result = b.rotate_coord_90_deg_clockwise(&test_case.input);
            assert_eq!(result, test_case.expected, "Failed to rotate coordinate: {:?}", test_case.input);
        })
    }
}
