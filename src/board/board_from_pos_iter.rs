use super::Board;
use position_types::*;
use std::iter::FromIterator;

impl FromIterator<PosCoords> for Board {
    fn from_iter<I: IntoIterator<Item = PosCoords>>(iter: I) -> Board {
        unimplemented!();
    }
}
