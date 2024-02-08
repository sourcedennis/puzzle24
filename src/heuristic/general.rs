// local imports
use crate::grid::Puzzle24Grid;


pub trait Heuristic {
  /// An admissible heuristic. It *underapproximates* the distance (in the search
  /// space) to the final puzzle.
  /// 
  /// 
  /// Design Decision: u8
  /// 
  /// No 24-puzzle needs more than 255 moves to the final solution.
  /// (TODO: source?)
  fn compute< P: Puzzle24Grid >( &self, v: &P ) -> u8;
}
