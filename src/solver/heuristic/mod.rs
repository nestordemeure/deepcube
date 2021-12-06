use crate::cube::Cube;
mod corners;
mod miniCube;
pub use corners::CornersHeuristic;

/// implemented by all heuristics to be used in algorithms such as A*
pub trait Heuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8;
}
