use {Board, PosCoords};

impl Board {
    /// Return a vector of Boards representing reflections of the current state.
    pub fn get_reflections(&self) -> Vec<Board> {
        vec![
            self.get_horizontal_reflection(),
            self.get_vertical_reflection(),
            self.get_inverse(),
        ]
    }

    /// Reflect the board horizontally on the y-axis.
    fn get_horizontal_reflection(&self) -> Board {
        self.queens
            .iter()
            .map(|pos| self.get_pos_horizontal_reflection(pos))
            .collect()
    }

    /// Reflect the board vertically on the y-axis.
    fn get_vertical_reflection(&self) -> Board {
        self.queens
            .iter()
            .map(|pos| self.get_pos_vertical_reflection(pos))
            .collect()
    }

    /// Reflect the board on both axes.
    fn get_inverse(&self) -> Board {
        self.queens
            .iter()
            .map(|pos| self.get_pos_inverse(pos))
            .collect()
    }

    /// Reflect a position horizontally on the x-axis.
    fn get_pos_horizontal_reflection(&self, pos: &PosCoords) -> PosCoords {
        let &(orig_x, y) = pos;
        let new_x = self.width - orig_x - 1;
        (new_x, y)
    }

    /// Reflect a position vertically on the y-axis.
    fn get_pos_vertical_reflection(&self, pos: &PosCoords) -> PosCoords {
        let &(x, orig_y) = pos;
        let new_y = self.height - orig_y - 1;
        (x, new_y)
    }

    /// Reflect a position vertically and horizontally.
    fn get_pos_inverse(&self, pos: &PosCoords) -> PosCoords {
        let &(orig_x, orig_y) = pos;
        let new_y = self.height - orig_y - 1;
        let new_x = self.width - orig_x - 1;
        (new_x, new_y)
    }
}

#[cfg(test)]
mod pos_reflect_tests {
    use {Board, PosCoords};

    struct RotateTestCase {
        input: PosCoords,
        expected_horizontal: PosCoords,
        expected_vertical: PosCoords,
        expected_inverse: PosCoords,
    }

    static TEST_CASES: &[RotateTestCase] = &[
        RotateTestCase {
            input: (0, 7),
            expected_horizontal: (7, 7),
            expected_vertical: (0, 0),
            expected_inverse: (7, 0),
        },
        RotateTestCase {
            input: (2, 6),
            expected_horizontal: (5, 6),
            expected_vertical: (2, 1),
            expected_inverse: (5, 1),
        },
    ];

    #[test]
    fn position_reflections_work() {
        let b = Board::new();
        TEST_CASES.iter().for_each(|test_case| {
            let &RotateTestCase {
                input,
                expected_horizontal,
                expected_vertical,
                expected_inverse,
            } = test_case;

            let actual_horizontal = b.get_pos_horizontal_reflection(&input);
            assert_eq!(actual_horizontal, expected_horizontal);

            let actual_inverse = b.get_pos_inverse(&input);
            assert_eq!(actual_inverse, expected_inverse);

            let actual_vertical = b.get_pos_vertical_reflection(&input);
            assert_eq!(actual_vertical, expected_vertical);
        })
    }
}
