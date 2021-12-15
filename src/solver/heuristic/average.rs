use serde::{Serialize, Deserialize};
use super::{Heuristic, MiddlesHeuristic, CornersHeuristic};
use crate::cube::Cube;

/// does an average of the middle and corner heuristics (rounded up)
/// this value is not as good as a maxmimum but I hypothesize that it will be more information rich
#[derive(Serialize, Deserialize)]
pub struct AverageHeuristic
{
    pub corners_heuristic: CornersHeuristic,
    pub middles_heuristic: MiddlesHeuristic
}

impl Heuristic for AverageHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        // computes the individual heuristics
        let corners_distance = self.corners_heuristic.optimistic_distance_to_solved(cube) as usize;
        let middles_distance_lower =
            self.middles_heuristic.optimistic_distance_to_solved_lower(cube) as usize;
        let middles_distance_upper =
            self.middles_heuristic.optimistic_distance_to_solved_upper(cube) as usize;
        // assemble the heuristics
        // we use usize to avoid overflows here
        let sum_distances = corners_distance + middles_distance_lower + middles_distance_upper;
        let average_ceil = (sum_distances + 2) / 3;
        average_ceil as u8
    }
}

impl AverageHeuristic
{
    /// initialize the heuristic
    pub fn new() -> AverageHeuristic
    {
        let corners_heuristic = CornersHeuristic::new();
        let middles_heuristic = MiddlesHeuristic::new();
        AverageHeuristic { corners_heuristic, middles_heuristic }
    }
}
