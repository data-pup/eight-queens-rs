use {Board, CoordList, PosCoords, Reflection};

impl Reflection for Board {
    fn get_horizontal_reflection(&self) -> CoordList {
        self.get_queen_positions()
            .iter()
            .map(|pos| self.get_pos_horizontal_reflection(pos))
            .collect()
    }

    fn get_vertical_reflection(&self) -> CoordList {
        self.get_queen_positions()
            .iter()
            .map(|pos| self.get_pos_vertical_reflection(pos))
            .collect()
    }

    fn get_inverse(&self) -> CoordList {
        self.get_queen_positions()
            .iter()
            .map(|pos| self.get_pos_inverse(pos))
            .collect()
    }
}

impl Board {
    fn get_pos_horizontal_reflection(&self, pos: &PosCoords) -> PosCoords {
        let &(orig_x, y) = pos;
        let new_x = self.width - orig_x - 1;
        (new_x, y)
    }

    fn get_pos_vertical_reflection(&self, pos: &PosCoords) -> PosCoords {
        let &(x, orig_y) = pos;
        let new_y = self.height - orig_y - 1;
        (x, new_y)
    }

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
