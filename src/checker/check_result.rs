/// Represents a given solution state.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CheckResult {
    pub has_conflict: bool,
    pub is_solved: bool,
    pub num_queens: u8,
    pub num_free_spaces: u8,
}

#[cfg(test)]
mod check_result_ordering_test {
    extern crate rand;
    use super::CheckResult;
    use rand::Rng;
    use std::cmp::Ordering;

    #[test]
    fn solution_is_gt_than_default() {
        let soln = create_solution_result();
        let def = create_default_board_check_result();
        assert!(soln > def);
    }

    #[test]
    fn partial_solution_is_gt_than_default() {
        let partial_soln = create_partial_solution_2_queens();
        let def = create_default_board_check_result();
        assert!(partial_soln > def);
    }

    #[test]
    fn sort_various_results() {
        let mut rng = rand::thread_rng();
        let num_trials = 100;
        let mut input = vec![
            create_partial_solution_4_queens_30_free(),
            create_default_board_check_result(),
            create_partial_solution_2_queens(),
            create_partial_solution_4_queens_50_free(),
            create_solution_result(),
        ];
        let expected = vec![
            create_solution_result(),
            create_partial_solution_4_queens_50_free(),
            create_partial_solution_4_queens_30_free(),
            create_partial_solution_2_queens(),
            create_default_board_check_result(),
        ];
        for i in 0..num_trials {
            rng.shuffle(&mut input);
            input.sort();
            assert_eq!(input, expected);
        }
    }

    fn create_solution_result() -> CheckResult {
        CheckResult {
            is_solved: true,
            has_conflict: false,
            num_queens: 8,
            num_free_spaces: 56,
        }
    }

    fn create_partial_solution_2_queens() -> CheckResult {
        CheckResult {
            is_solved: false,
            has_conflict: false,
            num_queens: 2,
            num_free_spaces: 62,
        }
    }

    fn create_partial_solution_4_queens_50_free() -> CheckResult {
        CheckResult {
            is_solved: false,
            has_conflict: false,
            num_queens: 4,
            num_free_spaces: 50,
        }
    }

    fn create_partial_solution_4_queens_30_free() -> CheckResult {
        CheckResult {
            is_solved: false,
            has_conflict: false,
            num_queens: 4,
            num_free_spaces: 30,
        }
    }

    fn create_default_board_check_result() -> CheckResult {
        CheckResult {
            is_solved: false,
            has_conflict: false,
            num_queens: 0,
            num_free_spaces: 64,
        }
    }
}
