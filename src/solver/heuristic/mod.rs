use crate::cube::Cube;
mod corners;
pub use corners::CornersHeuristic;
mod middles;
pub use middles::MiddlesHeuristic;
mod permutations;

/// implemented by all heuristics to be used in algorithms such as A*
pub trait Heuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8;
}
