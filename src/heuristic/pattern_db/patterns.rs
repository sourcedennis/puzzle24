
/// We have 4 patterns, each with 6 tiles. Note that this matches *tiles* (not
/// cells) to the locations in their respective pattern. (Coincidentally they
/// overlap for the final puzzle)
/// 
/// This pattern structure is taken from Figure 5 of the paper mentioned in
/// `mod.rs`.
/// 
/// Observe that pattern 2 and 3 are rotations of pattern 1. (9,9) represents
/// the gap.
pub(crate) const PATTERN_IDX: [(u8,u8);25] =
  [ (9,9), (0,0), (2,5), (2,4), (2,3)
  , (0,1), (0,2), (2,2), (2,1), (2,0)
  , (0,3), (0,4), (0,5), (3,2), (3,5)
  , (1,0), (1,1), (1,2), (3,1), (3,4)
  , (1,3), (1,4), (1,5), (3,0), (3,3)
  ];

/// The mirror (along the diagonal) of [`PATTERN_IDX`].
pub(crate) const MIRROR_PATTERN_IDX: [(u8,u8);25] =
  [ (9,9), (0,1), (0,3), (1,0), (1,3)
  , (0,0), (0,2), (0,4), (1,1), (1,4)
  , (2,5), (2,2), (0,5), (1,2), (1,5)
  , (2,4), (2,1), (3,2), (3,1), (3,0)
  , (2,3), (2,0), (3,5), (3,4), (3,3)
  ];



#[inline]
fn rot_right( (x,y): (u8,u8) ) -> (u8, u8) {
  (4-y, x)
}

/// Mirrors along the matrix diagonal. We use it to map from [`PATTERN_IDX`] to
/// [`MIRROR_PATTERN_IDX`].
#[inline]
pub fn mirror( (x,y): (u8,u8) ) -> (u8, u8) {
  (y, x)
}

/// 
/// 
/// Note that pattern 2 is a rotation of pattern 1.
#[inline]
pub fn pat2_to_pat1( (x,y): (u8,u8) ) -> (u8, u8) {
  rot_right( rot_right( (x, y) ) )
}

///
/// 
/// Note that pattern 3 is a rotation of pattern 1.
#[inline]
pub fn pat3_to_pat1( (x,y): (u8, u8) ) -> (u8, u8) {
  rot_right( (x,y) )
}
