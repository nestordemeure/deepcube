use serde::{Serialize, Deserialize};
use super::{Heuristic, MiddlesHeuristic, CornersHeuristic};
use crate::cube::Cube;

/// maximum between the corners heuristic and the middles heuristic
#[derive(Serialize, Deserialize)]
pub struct KorfHeuristic
{
    corners_heuristic: CornersHeuristic,
    middles_heuristic: MiddlesHeuristic
}

impl Heuristic for KorfHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        let corners_distance = self.corners_heuristic.optimistic_distance_to_solved(cube);
        let middles_distance = self.middles_heuristic.optimistic_distance_to_solved(cube);
        corners_distance.max(middles_distance)
    }
}

impl KorfHeuristic
{
    /// initialize the heuristic
    pub fn new() -> KorfHeuristic
    {
        let corners_heuristic = CornersHeuristic::new();
        let middles_heuristic = MiddlesHeuristic::new();
        KorfHeuristic { corners_heuristic, middles_heuristic }
    }
}
