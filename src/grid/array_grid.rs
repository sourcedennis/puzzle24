// local imports
use super::general::*;


#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct ArrayGrid24( [PuzzleTile; 25] );

impl ArrayGrid24 {
  #[inline]
  pub const fn from_cells_unchecked( cells: [u8; 25] ) -> Self {
    let mut out = [PuzzleTile::from_const( 0u8 ); 25];

    // Note: for-loops are disallowed in `const fn`s
    let mut cell_idx = 0;
    while cell_idx < 25 {
      let tile = cells[ cell_idx ];
      // Note that this call performs input validation.
      out[ cell_idx ] = PuzzleTile::from_const( tile as u8 );
      cell_idx += 1;
    }

    ArrayGrid24( out )
  }
}

impl Puzzle24Grid for ArrayGrid24 {
  #[inline]
  fn get_tile( &self, cell: PuzzleCell ) -> PuzzleTile {
    // This is safe under our range invariant on `PuzzleCell`
    unsafe { *self.0.get_unchecked( u8::from( cell ) as usize ) }
  }

  #[inline]
  fn set_tile( &mut self, cell: PuzzleCell, tile: PuzzleTile ) {
    let cell_ref = unsafe { self.0.get_unchecked_mut( u8::from( cell ) as usize ) };
    *cell_ref = tile;
  }
}

impl From< [u8; 25] > for ArrayGrid24 {
  fn from( v: [u8; 25] ) -> Self {
    let mut out = [PuzzleTile::from( 0u8 ); 25];

    // Note: for-loops are disallowed in `const fn`s
    for cell_idx in 0..25 {
      let tile = v[ cell_idx ];
      // note that `PuzzleTile::from` does input validation
      out[ cell_idx ] = PuzzleTile::from( tile as u8 );
    }

    ArrayGrid24( out )
  }
}
