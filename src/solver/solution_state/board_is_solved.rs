use Queens;

/// Check if the board is currently a solution to the 8 queens problem.
pub fn check_is_solved(b: &Queens) -> bool {
    let q_positions = b.get_queen_positions();
    if q_positions.len() < 8 {
        return false;
    } else if q_positions.len() > 8 {
        panic!("More than 8 queens are on the board.")
    }

    for &curr_pos in q_positions.iter() {
        let _moves = b.get_queen_moves(curr_pos);
        unimplemented!(); // Check if any of the positions contain a queen.
    }
    unimplemented!(); // Fixup.
}
