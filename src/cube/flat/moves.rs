//! Represents ways to twist a cube
//!
//! We convert classical notations in permutation tables
//!
//! See this website for the classical notations:
//! http://www.rubiksplace.com/move-notations/
use super::super::moves::Move;
use super::super::sizes::NB_SQUARES_CUBE;
use super::super::Cube;

/// move compiled into a permutation table
/// the compilation step is expensive but needs to be run only once
pub struct FlatMove
{
    pub description: Move,
    pub permutation: [usize; NB_SQUARES_CUBE]
}

impl FlatMove
{
    /// returns a vector containing all possible compiled moves
    pub fn all_moves(preserve_orientation: bool) -> Vec<FlatMove>
    {
        Move::all_moves().into_iter().map(|m| FlatMove::compile(m, preserve_orientation)).collect()
    }

    /// takes a move and compiles it down to a permutation table (a CompiledMove)
    /// if `preserve_orientation` is set to `true`, move to middle layers will instead be counter moves to lateral layers
    /// NOTE: this step is too expensive to be run whenever a move needs to be applied, instead it is meant as a preparation step
    fn compile(description: Move, preserve_orientation: bool) -> FlatMove
    {
        unimplemented!();
        // applies the move to a sorted cube, making it unsorted
        let cube_twisted = if preserve_orientation
        {
            Cube::solved().apply_move_orientation_preserving_unsorted(description)
        }
        else
        {
            Cube::solved().apply_move_unsorted(description)
        };
        // builds indexes that will be returned at the end
        let mut permutation_inversed: Vec<usize> = (0..NB_SQUARES_CUBE).collect();
        // finds the permutation that gets us from the moved position to the initial, sorted, position
        permutation_inversed.sort_unstable_by_key(|i| cube_twisted.blocks[*i].position);
        // inverses the permutation to get from sorted to unsorted
        let mut permutation: [usize; NB_SQUARES_CUBE] = [0; NB_SQUARES_CUBE];
        for i in 0..NB_SQUARES_CUBE
        {
            permutation[permutation_inversed[i]] = i;
        }
        // returns the move
        FlatMove { description, permutation }
        // TODO
        // one big problem here is that we are working with blocks and not indexes on a flattened cube
        // the best solution might be to unflatten an array of indexes, apply the move and flatten the result thus obtaining a permutation
    }
}
