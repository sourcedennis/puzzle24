///! This implements disjoint pattern databases as taken from:
/// "Disjoint pattern database heuristics" by Richard E. Korf, and Ariel Felner
/// 
/// We mostly hard-code the "default" shaped patterns ([`PATTERN_IDX`]) for
/// performance reasons.

mod patterns;
mod packings;
mod db;
mod helpers;

pub use db::{Pattern6Puzzle, DefaultPatternDB};
