// local imports
use crate::grid::ArrayGrid24;

/// Ten 24-puzzles taken from the paper:
/// "Finding Optimal Solutions to the Twenty-Four Puzzle" by Richard E. Korf and Larry A. Taylor
pub const EXAMPLE_PUZZLES: [ArrayGrid24; 10] =
  [ ArrayGrid24::from_cells_unchecked( [17,1,20,9,16, 2,22,19,14,5, 15,21,0,3,24, 23,18,13,12,7, 10,8,6,4,11] ) // optimal = 100
  , ArrayGrid24::from_cells_unchecked( [14,5,9,2,18, 8,23,19,12,17, 15,0,10,20,4, 6,11,21,1,7, 24,3,16,22,13] ) // optimal = 95
  , ArrayGrid24::from_cells_unchecked( [7,13,11,22,12, 20,1,18,21,5, 0,8,14,24,19, 9,4,17,16,10, 23,15,3,2,6] ) // optimal = 108
  , ArrayGrid24::from_cells_unchecked( [18,14,0,9,8, 3,7,19,2,15, 5,12,1,13,24, 23,4,21,10,20, 16,22,11,6,17] ) // optimal = 98
  , ArrayGrid24::from_cells_unchecked( [2,0,10,19,1, 4,16,3,15,20, 22,9,6,18,5, 13,12,21,8,17, 23,11,24,7,14] ) // optimal = 101
  , ArrayGrid24::from_cells_unchecked( [16,5,1,12,6, 24,17,9,2,22, 4,10,13,18,19, 20,0,23,7,21, 15,11,8,3,14] ) // optimal = 96
  , ArrayGrid24::from_cells_unchecked( [21,22,15,9,24, 12,16,23,2,8, 5,18,17,7,10, 14,13,4,0,6, 20,11,3,1,19] ) // optimal = 104
  , ArrayGrid24::from_cells_unchecked( [6,0,24,14,8, 5,21,19,9,17, 16,20,10,13,2, 15,11,22,1,3, 7,23,4,18,12] ) // optimal = 97
  , ArrayGrid24::from_cells_unchecked( [3,2,17,0,14, 18,22,19,15,20, 9,7,10,21,16, 6,24,23,8,5, 1,4,11,12,13] ) // optimal = 113
  , ArrayGrid24::from_cells_unchecked( [23,14,0,24,17, 9,20,21,2,18, 10,13,22,1,3, 11,4,16,6,5, 7,12,8,15,19] ) // optimal = 114
  ];
