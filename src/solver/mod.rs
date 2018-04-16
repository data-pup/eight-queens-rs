use board::Board;
use {Queens, UncontestedSpaces};

pub struct Solver {
    board: Board,
    is_solved: bool,
}

impl Solver {
    fn new(b: Board) {
        unimplemented!();
    }

    fn check_is_solved(b: &Board) -> bool {
        let q_positions = b.get_queen_positions();
        if q_positions.len() < 8 {
            return false;
        } else if q_positions.len() > 8 {
            panic!("More than 8 queens are on the board.")
        }

        for &curr_pos in q_positions.iter() {
            let moves = b.get_queen_moves(curr_pos);
            unimplemented!(); // Check if any of the positions contain a queen.
        }

        unimplemented!();
    }
}
