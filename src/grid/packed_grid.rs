// local imports
use super::general::*;


/// We pack cells as 5 bits each, requiring a total of 125 bits.
/// 
/// This is often slower than `ArrayGrid24` (on my machine ~30%, for some
/// problems). However, it requires little space, and may be preferable when
/// storing many states in memory (e.g., with Breadth-first search).
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct PackedGrid24( u128 );

impl PackedGrid24 {
  #[inline]
  pub const fn from_cells_unchecked( cells: [u8; 25] ) -> Self {
    let mut out = 0u128;
    let mut cell_idx = 0;

    // Note: for-loops are disallowed in `const fn`s
    while cell_idx < 25 {
      let tile = cells[ cell_idx ];
      out |= ( tile as u128 ) << ( 5 * cell_idx );
      cell_idx += 1;
    }

    PackedGrid24( out )
  }
}

impl Puzzle24Grid for PackedGrid24 {
  #[inline]
  fn get_tile( &self, cell: PuzzleCell ) -> PuzzleTile {
    // Note that we rely upon the invariant on `PuzzleCell` being in [0..24].
    let cell = u8::from( cell );
    let shift = cell * 5;
    let t = ( ( self.0 >> shift ) as u8 ) & 0b1_1111;
    PuzzleTile::from( t )
  }

  #[inline]
  fn set_tile( &mut self, cell: PuzzleCell, tile: PuzzleTile ) {
    // Note that we rely upon the invariant on `PuzzleCell` being in [0..24].
    let cell = u8::from( cell );
    let shift = cell * 5;
    let zero_mask = !( 0b1_1111u128 << shift );
    let new_tile_val = ( u8::from( tile ) as u128 ) << shift;
    self.0 = ( self.0 & zero_mask ) | new_tile_val;
  }

  #[inline]
  fn clear_cell( &mut self, cell: PuzzleCell ) {
    // Equivalent to (but more efficient):
    // self.set_tile( cell, PuzzleTile::GAP );
    let cell = u8::from( cell );
    let mask = !( 0b1_1111u128 << ( cell * 5 ) );
    self.0 &= mask;
  }

  #[inline]
  fn set_gap_tile_unchecked( &mut self, gap_cell: PuzzleCell, tile: PuzzleTile ) {
    // Precondition: The cell `gap_cell` currently contains the gap tile.
    // Equivalent to (but more efficient):
    // self.set_tile( cell, PuzzleTile::GAP );
    let tile = u8::from( tile );
    let gap_cell = u8::from( gap_cell );
    let new_tile_val = ( tile as u128 ) << ( gap_cell * 5 );
    // note that, if the cell does not currently contain the gap, this produces
    // arbitrary results. Hence, we strictly rely upon our precondition.
    self.0 = self.0 | new_tile_val;
  }
}

impl From< [u8; 25] > for PackedGrid24 {
  fn from( cells: [u8; 25] ) -> Self {
    let mut out = 0u128;

    for cell_idx in 0..25 {
      let tile = cells[ cell_idx ];
      debug_assert!( tile < 25 ); // input validation
      out |= ( tile as u128 ) << ( 5 * cell_idx );
    }

    PackedGrid24( out )
  }
}
