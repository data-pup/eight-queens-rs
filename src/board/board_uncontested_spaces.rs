use std::collections::HashSet;

use UncontestedSpaces;
use position_types::*;
use super::Board;

impl UncontestedSpaces for Board {
    fn get_uncontested_spaces(&self) -> HashSet<PosCoords> {
        unimplemented!();
    }
}

#[cfg(test)]
mod board_uncontested_spaces_tests {
    // use position_types::*;
    // use super::Board;

    #[test]
    fn it_works() {
        assert_eq!(1, 2, "To do...");
    }
}