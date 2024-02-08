// local imports
use crate::grid::{ArrayGrid24, Dir, Puzzle24Grid, PuzzleCell, PuzzleTile};
use super::super::general::Heuristic;
use super::patterns::*;
use super::packings::*;
use super::helpers::*;


#[derive(Copy, Clone)]
pub struct Pattern6Puzzle {
  /// A grid with 6 tiles and the gap. All other tiles are set to 7.
  /// 
  /// We use `ArrayCells24` over `PackedCells24`, because it's faster (but takes
  /// more memory, which is fine when building relatively-small DBs).
  grid: ArrayGrid24,
  /// The cell (in `grid`) containing the gap.
  gap_cell: PuzzleCell
}

const NON_PATTERN_CELL: u8 = 7;

impl Pattern6Puzzle {
  /// Pattern 0 in [`PATTERN_IDX`]
  pub const DEFAULT_PATTERN0: Pattern6Puzzle =
    Pattern6Puzzle::extract_pattern_unchecked( PATTERN_IDX, 0 );

  /// Pattern 1 in [`PATTERN_IDX`]
  /// 
  /// Note that, we can obtain pattern 2 and 3 through rotation.
  pub const DEFAULT_PATTERN1: Pattern6Puzzle =
    Pattern6Puzzle::extract_pattern_unchecked( PATTERN_IDX, 1 );

  /// Extracts an individual pattern from a puzzle pattern (e.g., such as
  /// [`PATTERN_IDX`]).
  #[inline]
  const fn extract_pattern_unchecked(
    pattern: [(u8,u8);25]
  , pattern_id: u8
  ) -> Pattern6Puzzle {
    let mut cell_idx = 0;
    let mut cells = [NON_PATTERN_CELL; 25];

    // Note that for-loops are disallowed in `const fn`s.
    while cell_idx < 25 {
      // note that, for the final puzzle, tiles are identical to cells
      let tile = cell_idx;

      let (p_id, p_idx) = pattern[ tile as usize ];

      if pattern_id == p_id {
        // Doing `+ 1`, because the index in a pattern starts at 0. However,
        // 0 is our gap.
        cells[ cell_idx ] = p_idx + 1;
      }

      cell_idx += 1;
    }

    let grid = ArrayGrid24::from_cells_unchecked( cells );
    // The gap, by the definition of 24-puzzles, is located at cell 0.
    // (This means `pattern[ 0 ] == (9,9)`, but disallowed that's in `const fn`)
    debug_assert!( pattern[ 0 ].0 == 9 && pattern[ 0 ].1 == 9 );
    let gap_cell = PuzzleCell::from_const( 0 );

    Pattern6Puzzle { grid, gap_cell }
  }

  /// If the move is possible, returns the new `Pattern6Puzzle` with a boolean
  /// indicating whether the move was from a pattern tile. (i.e., it wasn't a
  /// tile that doesn't belong to the pattern).
  /// 
  /// Note that the provided [`Dir`] represents the "inverted direction" of the
  /// moved tile. (Instead, it represents the direction in which the gap moves).
  #[inline]
  pub fn step_inv( &self, inv_dir: Dir ) -> Option< (Self, bool) > {
    let adj_cell = self.gap_cell.step( inv_dir )?;

    // The tile we move into the gap
    let moved_tile = self.grid.get_tile( adj_cell );
    let mut new_grid = self.grid.clone( );
    new_grid.set_tile( self.gap_cell, moved_tile );
    new_grid.clear_cell( adj_cell );

    debug_assert!( u8::from( moved_tile ) != 0 );

    let new_p = Pattern6Puzzle { grid: new_grid, gap_cell: adj_cell };

    Some( (new_p, u8::from( moved_tile ) != NON_PATTERN_CELL) )
  }

  #[inline]
  pub fn step( &self, dir: Dir ) -> Option< (Self, bool) > {
    self.step_inv( dir.inv( ) )
  }

  #[inline]
  fn to_u32( &self ) -> Pattern6Packing {
    let mut packing_u32 = Pattern6Packing::default( );

    for cell_idx in 0..25 {
      let cell_idx = PuzzleCell::from_const( cell_idx );
      let tile = self.grid.get_tile( cell_idx );
      let tile_u8 = u8::from( tile );

      if tile_u8 >= 1 && tile_u8 <= 6 { // we do the gap separately
        packing_u32.set_tile_unchecked( tile_u8 - 1, cell_idx );
      }
    }
    packing_u32
  }

  /// Produces a database of size `244_140_625`.
  /// 
  /// Note that this takes some time (i.e., 10 min on my system).
  /// 
  /// Note that this is only a partial DB, for one pattern in the set.
  pub fn build_pattern_db( &self ) -> Vec< u8 > {
    let mut distances = vec![0xFF; 244_140_625]; // 25^6
    // Note that we will only visit 25!/(25-7)! nodes (i.e., 2_422_728_000).
    let mut visited: BitSet = BitSet::new( 6_103_515_625 ); // 25^7
    let mut depth = 0;
    // the queue at depth `depth`.
    let mut curr_queue: Vec< Pattern6Puzzle > = Vec::new( );
    curr_queue.push( *self );
    // the queue at depth `depth + 1`.
    let mut next_queue: Vec< Pattern6Puzzle > = Vec::new( );

    while !curr_queue.is_empty( ) {
      while let Some( p ) = curr_queue.pop( ) {
        // this maps all 6 pattern tiles to their cells
        let num_u32 = p.to_u32( );
        // this maps all 6 pattern tiles + the gap to their cells
        let num_u64 = Pattern7Packing::from_u32( num_u32, p.gap_cell );
  
        if unsafe { visited.insert_unchecked( u64::from( num_u64 ) as usize ) } {
          let distances_ref =
            unsafe { distances.get_unchecked_mut( u32::from( num_u32 ) as usize ) };
          if *distances_ref == 0xFF {
            *distances_ref = depth;
          }
  
          for dir in Dir::ALL {
            if let Some( (next_p, is_pattern_tile) ) = p.step_inv( dir ) {
              if is_pattern_tile { // count the move (i.e., at `depth + 1`)
                next_queue.push( next_p );
              } else { // don't count the move (i.e., at `depth`)
                curr_queue.push( next_p );
              }
            }
          }
        }
      }
      // assert: curr_queue.is_empty( )
      depth += 1;
      std::mem::swap( &mut curr_queue, &mut next_queue );
    }
    distances
  }

  #[inline]
  pub fn extract_u32s< P: Puzzle24Grid >( p: &P ) -> ([u32; 4], [u32; 4]) {
    // Matches pattern tiles to their cell in the puzzle
    let mut tile_to_cell = [Pattern6Packing::default( ); 4];
    let mut mirror_tile_to_cell = [Pattern6Packing::default( ); 4];

    for cell_y in 0..5 {
      for cell_x in 0..5 {
        let tile = p.get_tile( (cell_x, cell_y).into( ) );

        if tile == PuzzleTile::GAP {
          continue;
        }
  
        let (pattern_id, pattern_idx) = PATTERN_IDX[ u8::from( tile ) as usize ];
        let pattern_id = pattern_id as usize;

        // map pattern 2 and 3 to rotations of pattern 1
        let out_cell_idx: PuzzleCell =
          match pattern_id {
            0|1 => (cell_x, cell_y).into( ), // In their regular orientation
            2   => pat2_to_pat1( (cell_x, cell_y) ).into( ),
            3   => pat3_to_pat1( (cell_x, cell_y) ).into( ),
            _   => panic!( )
          };
        tile_to_cell[ pattern_id ].set_tile_unchecked( pattern_idx, out_cell_idx );


        let (mir_pattern_id, mir_pattern_idx) = MIRROR_PATTERN_IDX[ u8::from( tile ) as usize ];
        let mir_pattern_id = mir_pattern_id as usize;
        let mir_out_cell_idx: PuzzleCell =
          match mir_pattern_id {
            0|1 => mirror( (cell_x, cell_y) ).into( ),
            2   => pat2_to_pat1( mirror( (cell_x, cell_y) ) ).into( ),
            3   => pat3_to_pat1( mirror( (cell_x, cell_y) ) ).into( ),
            _   => panic!( )
          };
        mirror_tile_to_cell[ mir_pattern_id ].set_tile_unchecked( mir_pattern_idx, mir_out_cell_idx );
      }
    }

    ( [ tile_to_cell[ 0 ].into( ), tile_to_cell[ 1 ].into( )
      , tile_to_cell[ 2 ].into( ), tile_to_cell[ 3 ].into( ) ]
    , [ mirror_tile_to_cell[ 0 ].into( ), mirror_tile_to_cell[ 1 ].into( )
      , mirror_tile_to_cell[ 2 ].into( ), mirror_tile_to_cell[ 3 ].into( ) ]
    )
  }
}

/// The "default" pattern DB, as given in [`PATTERN_IDX`]. (And described in the
/// paper referenced by `mod.rs`)
/// 
/// We obtain pattern 2 and 3 by rotating pattern 1. We additionally compute the
/// value of the mirrored DB, with which we compute the maximum. (See
/// [`DefaultPatternDB::compute()`])
pub struct DefaultPatternDB {
  default_pattern0: Vec< u8 >,
  default_pattern1: Vec< u8 >
}

impl DefaultPatternDB {
  /// 
  /// 
  /// WARNING: Producing `pattern0` and `pattern1`
  /// 
  /// Note that the input patterns must be produce by [`Pattern6Puzzle::build_pattern_db()`].
  /// (On [`Pattern6Puzzle::DEFAULT_PATTERN0`] and [`Pattern6Puzzle::DEFAULT_PATTERN1`],
  /// respectively).
  /// 
  /// However, note that we don't do so internally, as building them takes a
  /// long time (see [`Pattern6Puzzle::build_pattern_db()`]). Instead, this
  /// gives the opportunity to store them to disk.
  #[inline]
  pub fn new( default_pattern0: Vec< u8 >, default_pattern1: Vec< u8 > ) -> Self {
    assert!( default_pattern0.len( ) == 244_140_625 && default_pattern1.len( ) == 244_140_625 );
    DefaultPatternDB { default_pattern0, default_pattern1 }
  }
}

impl Heuristic for DefaultPatternDB {
  fn compute< P: Puzzle24Grid >( &self, p: &P ) -> u8 {
    let (p0, p1) = Pattern6Puzzle::extract_u32s( p );

    let dis_a0 = self.default_pattern0[ p0[ 0 ] as usize ];
    let dis_a1 = self.default_pattern1[ p0[ 1 ] as usize ];
    let dis_a2 = self.default_pattern1[ p0[ 2 ] as usize ];
    let dis_a3 = self.default_pattern1[ p0[ 3 ] as usize ];
    let dis_a = dis_a0 + dis_a1 + dis_a2 + dis_a3;

    // in mirrored patterns
    let dis_b0 = self.default_pattern0[ p1[ 0 ] as usize ];
    let dis_b1 = self.default_pattern1[ p1[ 1 ] as usize ];
    let dis_b2 = self.default_pattern1[ p1[ 2 ] as usize ];
    let dis_b3 = self.default_pattern1[ p1[ 3 ] as usize ];
    let dis_b = dis_b0 + dis_b1 + dis_b2 + dis_b3;

    // both `dis_a` and `dis_b` are (independently) admissible heuristic values
    // so take their maximum
    dis_a.max( dis_b )
  }
}

impl From< DefaultPatternDB > for (Vec< u8 >, Vec< u8 >) {
  #[inline]
  fn from( v: DefaultPatternDB ) -> Self {
    ( v.default_pattern0, v.default_pattern1 )
  }
}
