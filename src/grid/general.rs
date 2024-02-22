// stdlib imports
use std::fmt;


/// A `Puzzle24` represents a 5x5 grid of cells containing tiles [0..24]. Tile 0
/// represents the gap.
/// 
/// We represent this as a trait to allow for different internal puzzle
/// representation (e.g., with different performance characteristics), to allow
/// for generic algorithm implementations.
/// 
/// 
/// # Invariants
/// 
/// * Contains exactly one gap tile
///     (note that we use this grid for complete puzzles and patterns)
/// * All tiles are in the range [0..24]
/// 
/// 
/// # Example
/// 
/// The *solved* puzzle is as follows:
/// 
/// ```ignore
///  0  1  2  3  4
///  5  6  7  8  9
/// 10 11 12 13 14
/// 15 16 17 18 19
/// 20 21 22 23 24
/// ```
pub trait Puzzle24Grid: From< [u8; 25] > + Clone{
  /// TODO
  /// 
  /// 
  /// # WARNING: `PuzzleCell` invariant
  /// 
  /// We rely upon the invariant on `PuzzleCell`'s being in the range [0..24].
  fn get_tile( &self, cell: PuzzleCell ) -> PuzzleTile;

  /// TODO
  /// 
  /// 
  /// # WARNING: `PuzzleCell` invariant
  /// 
  /// We rely upon the invariant on `PuzzleCell`'s being in the range [0..24].
  fn set_tile( &mut self, cell: PuzzleCell, tile: PuzzleTile );

  /// Set the a cell to be the gap tile.
  /// 
  /// # Rationale
  /// 
  /// This is equivalent to:
  /// ```
  /// self.set_tile( cell, PuzzleTile::GAP );
  /// ```
  /// However, overriding this may allow for a more efficient implementation.
  #[inline]
  fn clear_cell( &mut self, cell: PuzzleCell ) {
    self.set_tile( cell, PuzzleTile::GAP );
  }

  ///
  /// Precondition: The cell `gap_cell` currently contains the gap tile.
  /// 
  /// # Rationale
  /// 
  /// This is equivalent to:
  /// ```
  /// self.set_tile( cell, PuzzleTile::GAP );
  /// ```
  /// However, overriding this may allow for a more efficient implementation.
  #[inline]
  fn set_gap_tile_unchecked( &mut self, gap_cell: PuzzleCell, tile: PuzzleTile ) {
    self.set_tile( gap_cell, tile );
  }

  /// Returns the puzzle cell containing the gap (i.e., [`PuzzleTile::GAP`]).
  /// 
  /// Note that this may iterate over all cells, and thus be somewhat expensive.
  /// Often, it is better to cache the gap cell externally.
  #[inline]
  fn find_gap( &self ) -> PuzzleCell {
    for cell_idx in 0..25 {
      let cell_idx = PuzzleCell::from( cell_idx );
      let tile = self.get_tile( cell_idx );
  
      if tile == PuzzleTile::GAP {
        return cell_idx;
      }
    }
    panic!( "Invalid puzzle (no gap)")
  }

  /// Helper to print the grid.
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!( f, "[" )?;

    for cell_idx in 0..24 {
      let cell_idx = PuzzleCell::from( cell_idx );
      let tile = self.get_tile( cell_idx );

      if tile == PuzzleTile::GAP { // it's the gap
        write!( f, "_, " )?;
      } else {
        write!( f, "{}, ", u8::from( tile ) )?;
      }
    }

    let final_tile = self.get_tile( PuzzleCell::from( 24 ) );
    if final_tile == PuzzleTile::GAP { // it's the gap
      write!( f, "_, " )?;
    } else {
      write!( f, "{}, ", u8::from( final_tile ) )?;
    }
    
    write!( f, "]" )
  }
}

/// A cell inside a 24-puzzle. The cell represents the fixed location within the
/// grid, which stores a tile.
/// 
/// (Note that a tile moves between cells)
/// 
/// Invariant: The value is in the range [0..24]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct PuzzleCell( u8 );

/// A tile, contained in a cell within a 24-puzzle.
/// 
/// (Note that a tile moves between cells)
/// 
/// Invariant: The value is in the range [0..24]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PuzzleTile( u8 );

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
  UP = 0, RIGHT = 1, DOWN = 2, LEFT = 3
}

impl PuzzleCell {
  /// `const fn` variant of [`From`] (for `u8`)
  #[inline]
  pub const fn from_const( v: u8 ) -> Self {
    // Check our invariant on `PuzzleTile`'s value range
    debug_assert!( v < 25 );
    PuzzleCell( v )
  }

  /// Returns the `PuzzleCell` in the given direction.
  #[inline]
  pub const fn step( &self, dir: Dir ) -> Option< Self > {
    debug_assert!( self.0 < 25 );
    match dir {
      Dir::UP =>
        if self.0 >= 5 { Some( PuzzleCell( self.0 - 5 ) ) } else { None },
      Dir::DOWN =>
        if self.0 < 20 { Some( PuzzleCell( self.0 + 5 ) ) } else { None },
      Dir::LEFT =>
        if self.0 % 5 >= 1 { Some( PuzzleCell( self.0 - 1 ) ) } else { None },
      Dir::RIGHT =>
        if self.0 % 5 <  4 { Some( PuzzleCell( self.0 + 1 ) ) } else { None },
    }
  }
}

impl From< PuzzleCell > for u8 {
  #[inline]
  fn from( v: PuzzleCell ) -> Self {
    v.0
  }
}

impl From< u8 > for PuzzleCell {
  #[inline]
  fn from( v: u8 ) -> Self {
    // Check our invariant on `PuzzleTile`'s value range
    debug_assert!( v < 25 );
    PuzzleCell( v )
  }
}

impl From< (u8, u8) > for PuzzleCell {
  #[inline]
  fn from( (x, y): (u8, u8) ) -> Self {
    // Check our invariant on `PuzzleTile`'s value range
    debug_assert!( x < 5 && y < 5 );
    PuzzleCell( y * 5 + x )
  }
}

impl PuzzleTile {
  pub const GAP: PuzzleTile = PuzzleTile( 0 );
  
  /// `const fn` variant of [`From`] (for `u8`)
  #[inline]
  pub const fn from_const( v: u8 ) -> Self {
    // Check our invariant on `PuzzleTile`'s value range
    debug_assert!( v < 25 );
    PuzzleTile( v )
  }
}

impl From< PuzzleTile > for u8 {
  #[inline]
  fn from( v: PuzzleTile ) -> Self {
    v.0
  }
}

impl From< u8 > for PuzzleTile {
  #[inline]
  fn from( v: u8 ) -> Self {
    // Check our invariant on `PuzzleTile`'s value range
    debug_assert!( v < 25 );
    PuzzleTile( v )
  }
}

impl Dir {
  pub const ALL: [Dir; 4] = [Dir::UP, Dir::RIGHT, Dir::DOWN, Dir::LEFT];

  #[inline]
  pub fn inv( &self ) -> Dir {
    match self {
      Dir::UP    => Dir::DOWN,
      Dir::RIGHT => Dir::LEFT,
      Dir::DOWN  => Dir::UP,
      Dir::LEFT  => Dir::RIGHT
    }
  }
}

impl From< Dir > for char {
  fn from( v: Dir ) -> Self {
    match v {
      Dir::UP    => 'U',
      Dir::RIGHT => 'R',
      Dir::DOWN  => 'D',
      Dir::LEFT  => 'L',
    }
  }
}
