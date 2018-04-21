use std::collections::HashSet;

/// Position index. Note: This may be made private?
pub type PosIndex = usize;
/// Position coordinates. Note: These are in (x, y) format.
pub type PosCoords = (u32, u32);
// A set of position coordinates representing the current queen positions.
pub type CoordSet = HashSet<PosCoords>;

/// Position errors. Thrown if a coordinate access attempt is out of bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PosError {
    OutOfBounds,
}

/// Module used to import the different position types, and the error enum
/// that can be returned in the event of an invalid coordinate pair.
pub mod position_types {
    pub use super::CoordSet;
    pub use super::PosCoords;
    pub use super::PosError;
    pub use super::PosIndex;
}
