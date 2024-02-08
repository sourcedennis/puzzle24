// stdlib imports
use std::fmt;
// local imports
use crate::grid::Puzzle24Grid;
use crate::heuristic::Heuristic;
use crate::grid::Dir;
use crate::{PuzzleCell, PuzzleTile};


pub fn solve_ita< H: Heuristic, G: Puzzle24Grid >( h: &H, p: G ) -> (usize, Vec< Dir >) {
  let mut stack = DfsStack::uninit( );
  let num_steps = solve_ita_state( &mut stack, h, p );
  let path = stack.inv_path.into_iter( ).map( |x| x.inv( ) ).collect( );
  (num_steps, path)
}

pub fn solve_dfs< H: Heuristic, G: Puzzle24Grid >( h: &H, p: G, max_depth: u8 ) -> (usize, Option< Vec< Dir > >) {
  let mut stack = DfsStack::uninit( );
  let (is_solved, num_steps) = solve_dfs_state( &mut stack, h, p, max_depth );
  let path =
    if is_solved {
      Some( stack.inv_path.into_iter( ).map( |x| x.inv( ) ).collect( ) )
    } else {
      None
    };
  (num_steps, path)
}

///
/// Invariant: grid[ gap_cell ] contains tile 0
pub struct Puzzle24< G: Puzzle24Grid > {
  grid: G,
  gap_cell: PuzzleCell
}

impl< G: Puzzle24Grid > fmt::Debug for Puzzle24< G > {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.grid.fmt( f )
  }
}

impl< G: Puzzle24Grid > Puzzle24< G > {
  pub fn new( grid: G ) -> Self {
    let gap_cell = grid.find_gap( );
    let out = Puzzle24 { grid, gap_cell };
    out
  }

  #[inline]
  pub fn grid( &self ) -> &G {
    &self.grid
  }

  #[inline]
  pub fn step_inv( &self, inv_dir: Dir ) -> Option< Self > {
    debug_assert!( self.grid.get_tile( self.gap_cell ) == PuzzleTile::GAP );
    let adj_cell = self.gap_cell.step( inv_dir )?;

    // The tile we move into the gap
    let moved_tile = self.grid.get_tile( adj_cell );
    
    let mut new_grid = self.grid.clone( );
    new_grid.set_tile( self.gap_cell, moved_tile );
    new_grid.clear_cell( adj_cell );
    let new_puzzle = Puzzle24 { grid: new_grid, gap_cell: adj_cell };
    debug_assert!( moved_tile != PuzzleTile::GAP, "{:?}", new_puzzle );

    Some( new_puzzle )
  }
}


fn solve_ita_state< H: Heuristic, G: Puzzle24Grid >( stack: &mut DfsStack< G >, h: &H, p: G ) -> usize {
  let mut num_steps_total = 0;
  let mut max_depth = 1;

  loop {
    // println!( "Now at depth {}", max_depth );
    let (is_solved, num_steps) = solve_dfs_state( stack, h, p.clone( ), max_depth );
    num_steps_total += num_steps;

    if is_solved {
      return num_steps_total;
    }

    max_depth += 2;
  }
}

#[inline]
pub fn solve_dfs_state< H: Heuristic, G: Puzzle24Grid >( stack: &mut DfsStack< G >, h: &H, p: G, max_depth: u8 ) -> (bool, usize) {
  let mut num_steps = 0;

  stack.init( Puzzle24::new( p ) );

  while let Some( (opt_path_top, (ref mut p, ref mut state)) ) = stack.top_mut( ) {
    let next_dir =
      match state {
        State::Dir( dir ) => *dir,
        State::Done => { unsafe { stack.pop( ); } continue; },
      };
    *state = state.next( );

    // No need to go back to the state from which we just came
    if Some( next_dir.inv( ) ) == opt_path_top {
      continue;
    }

    if let Some( next_p ) = p.step_inv( next_dir ) {
      num_steps += 1;

      let fx = stack.inv_path.len( ) as u8 + 1; // the path length, with `next_dir` added
      let hx = h.compute( &next_p.grid ); // a lower-bound on our remaining distance
      let gx = fx + hx; // a lower-bound on our total path length

      if gx <= max_depth {
        unsafe { stack.push_unchecked( next_dir, next_p ); }

        if hx == 0 { // we're at the solution
          return (true, num_steps)
        }
      }
    }
  }

  (false, num_steps)
}

///
/// Invariant: stack.len( ) == path.len( ) + 1
pub struct DfsStack< G: Puzzle24Grid > {
  stack: Vec< (Puzzle24< G >, State) >,
  /// The path with "inverted" directions. (i.e., it represents the movements of
  /// the gap)
  inv_path: Vec< Dir >,
}

impl< G: Puzzle24Grid > DfsStack< G > {
  pub fn uninit( ) -> Self {
    // We represent path lengths by `u8`, because paths can never be longer than
    // `255`. (Actually, less than that, TODO)
    DfsStack { stack: Vec::with_capacity( 256 ), inv_path: Vec::with_capacity( 255 ) }
  }

  pub fn inv_path( &self ) -> &[Dir] {
    &self.inv_path
  }

  pub fn init( &mut self, frame_puzzle: Puzzle24< G > ) {
    self.stack.clear( );
    self.inv_path.clear( );
    
    self.stack.push( ( frame_puzzle, State::new( ) ) );
  }

  #[inline]
  pub fn top_mut( &mut self ) -> Option< (Option< Dir >, &mut (Puzzle24< G >, State)) > {
    if self.stack.is_empty( ) {
      None
    } else {
      let top_dir = self.top_dir( );
      Some( ( top_dir, unsafe { self.top_unchecked_mut( ) } ) )
    }
  }

  #[inline]
  pub fn top_dir( &self ) -> Option< Dir > {
    if self.inv_path.is_empty( ) {
      None
    } else {
      // Some( unsafe { *self.inv_path.get_unchecked( self.inv_path.len( ) - 1 ) } )
      Some( self.inv_path[ self.inv_path.len( ) - 1 ] )
    }
  }

  #[inline]
  pub unsafe fn top_unchecked_mut( &mut self ) -> &mut (Puzzle24< G >, State) {
    debug_assert!( !self.stack.is_empty( ) );
    let stack_len = self.stack.len( );
    self.stack.get_unchecked_mut( stack_len - 1 )
  }

  #[inline]
  pub unsafe fn push_unchecked( &mut self, inv_dir: Dir, p: Puzzle24< G > ) {
    // The same as: `self.stack.push( (p, s) )`
    debug_assert!( self.stack.len( ) < self.stack.capacity( ) );
    let ptr = self.stack.as_mut_ptr( ).add( self.stack.len( ) );
    ptr.write( ( p, State::new( ) ) );
    self.stack.set_len( self.stack.len( ) + 1 );

    // The same as: `self.path.push( dir )`
    debug_assert!( self.inv_path.len( ) < self.inv_path.capacity( ) );
    let ptr = self.inv_path.as_mut_ptr( ).add( self.inv_path.len( ) );
    ptr.write( inv_dir );
    self.inv_path.set_len( self.inv_path.len( ) + 1 );
  }

  #[inline]
  pub unsafe fn pop( &mut self ) {
    self.inv_path.pop( );
    debug_assert!( !self.stack.is_empty( ) );
    self.stack.set_len( self.stack.len( ) - 1 );
    // drop the frame. (this should be a no-op)
    std::ptr::drop_in_place( self.stack.as_mut_ptr( ).add( self.stack.len( ) ) );
  }
}

#[derive(Clone, Copy, Debug)]
pub enum State {
  Dir( Dir ),
  Done
}

impl State {
  #[inline]
  pub fn new( ) -> Self {
    State::Dir( Dir::UP )
  }

  #[inline]
  pub fn next( self ) -> Self {
    match self {
      State::Dir( Dir::UP )    => State::Dir( Dir::RIGHT ),
      State::Dir( Dir::RIGHT ) => State::Dir( Dir::DOWN ),
      State::Dir( Dir::DOWN )  => State::Dir( Dir::LEFT ),
      State::Dir( Dir::LEFT )  => State::Done,
      State::Done => panic!( ),
    }
  }
}
