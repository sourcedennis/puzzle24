// local imports
use crate::grid::PuzzleCell;


/// Packs 6 cells into a `u32` (less than 25^6 -- i.e., 0..244_140_625).
/// 
/// Note that this is still not the tightest packing, as that would be
/// 25!/(25-6)! (which is in the range 0..127_512_000). However, that adds
/// additional computation overhead (and this still fits nicely in memory).
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Pattern6Packing( u32 );

impl Pattern6Packing {
  /// Precondition: `pattern_tile_idx` was not previously set
  #[inline]
  pub fn set_tile_unchecked( &mut self, pattern_tile_idx: u8, cell: PuzzleCell ) {
    debug_assert!( pattern_tile_idx < 6 );
    let cell = u8::from( cell );
    self.0 += POW25[ pattern_tile_idx as usize ] * ( cell as u32 );
  }
}

impl From< Pattern6Packing > for u32 {
  #[inline]
  fn from( v: Pattern6Packing ) -> Self {
    v.0
  }
}


/// Packs 7 cells into a `u32` (less than 25^7 -- i.e., 0..6_103_515_625).
/// 
/// Note that this is still not the tightest packing, as that would be
/// 25!/(25-7)! (which is in the range 0..2_422_728_000). However, that adds
/// additional computation overhead (and this still fits nicely in memory).
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Pattern7Packing( u64 );

impl Pattern7Packing {
  /// 
  /// 
  /// Precondition: `pattern_tile_idx` was not previously set
  #[inline]
  pub fn set_tile_unchecked( &mut self, pattern_tile_idx: u8, cell: PuzzleCell ) {
    debug_assert!( pattern_tile_idx < 7 );
    let cell = u8::from( cell );
    self.0 += ( POW25[ pattern_tile_idx as usize ] as u64 ) * ( cell as u64 );
  }

  #[inline]
  pub fn from_u32( v: Pattern6Packing, cell: PuzzleCell ) -> Self {
    let mut out = Pattern7Packing::from( v );
    out.set_tile_unchecked( 6, cell );
    out
  }
}

impl From< Pattern7Packing > for u64 {
  #[inline]
  fn from( v: Pattern7Packing ) -> Self {
    v.0
  }
}

impl From< Pattern6Packing > for Pattern7Packing {
  #[inline]
  fn from( v: Pattern6Packing ) -> Self {
    Pattern7Packing( v.0 as u64 )
  }
}


/// Powers of 25
/// 
/// We keep this to avoid recomputing these powers repeatedly.
const POW25: [u32; 7] = [1, 25, 625, 15_625, 390_625, 9_765_625, 244_140_625];
