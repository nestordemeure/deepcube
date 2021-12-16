use serde::{Serialize, Deserialize};
use super::{Heuristic, UpperMiddleHeuristic, LowerMiddleHeuristic, CornerHeuristic};
use crate::cube::Cube;

/// does an average of the middle and corner heuristics (rounded up)
/// this value is not as good as a maxmimum but I hypothesize that it will be more information rich
#[derive(Serialize, Deserialize)]
pub struct AverageHeuristic
{
    pub corners_heuristic: CornerHeuristic,
    pub lower_middles_heuristic: LowerMiddleHeuristic,
    pub upper_middles_heuristic: UpperMiddleHeuristic
}

impl Heuristic for AverageHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        // computes the individual heuristics
        let corners_distance = self.corners_heuristic.optimistic_distance_to_solved(cube);
        let lower_middles_distance = self.lower_middles_heuristic.optimistic_distance_to_solved(cube);
        let upper_middles_distance = self.upper_middles_heuristic.optimistic_distance_to_solved(cube);
        // assemble the heuristics
        // we use usize to avoid overflows here
        let sum_distances = corners_distance + lower_middles_distance + upper_middles_distance;
        //let average_ceil = (sum_distances + 2) / 3;
        //average_ceil
        sum_distances
    }
}

impl AverageHeuristic
{
    /// initialize the heuristic
    pub fn new() -> AverageHeuristic
    {
        let corners_heuristic = CornerHeuristic::new();
        let lower_middles_heuristic = LowerMiddleHeuristic::new();
        let upper_middles_heuristic = UpperMiddleHeuristic::new();
        AverageHeuristic { corners_heuristic, lower_middles_heuristic, upper_middles_heuristic }
    }
}
