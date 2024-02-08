///! A module with various representations for 24-puzzle grids.
/// (See `Puzzle24`).

// Contains the `Puzzle24` trait
mod general;
mod packed_grid;
mod array_grid;

pub use general::*;
pub use packed_grid::*;
pub use array_grid::*;
