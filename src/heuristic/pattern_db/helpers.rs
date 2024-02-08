// stdlib imports
use std::mem::size_of;


/// A set of `usize`s, which we represents as bits.
pub struct BitSet( Vec< usize > );

impl BitSet {
  pub fn new( capacity: usize ) -> Self {
    let num_usizes = div_ceil( capacity, size_of::< usize >( ) );
    BitSet( vec![ 0; num_usizes ] )
  }

  /// Insert an element, without checking whether it is within capacity.
  #[inline]
  pub unsafe fn insert_unchecked( &mut self, v: usize ) -> bool {
    let idx = v / size_of::< usize >( );
    let bit_num = v % size_of::< usize >( );

    let mask: usize = 1 << bit_num;
    let v_idx_ref = unsafe { self.0.get_unchecked_mut( idx ) };

    if ( *v_idx_ref & mask ) != 0 {
      false
    } else {
      *v_idx_ref |= mask;
      true
    }
  }
}

#[inline]
fn div_ceil( x: usize, y: usize ) -> usize {
  if x % y == 0 {
    x / y
  } else {
    x / y + 1
  }
}
