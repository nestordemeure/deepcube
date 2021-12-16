use serde::{Serialize, Deserialize};
use super::{Heuristic, UpperMiddleHeuristic, LowerMiddleHeuristic, CornerHeuristic};
use crate::cube::Cube;

/// maximum between the corners heuristic and the middles heuristic
#[derive(Serialize, Deserialize)]
pub struct KorfHeuristic
{
    pub corners_heuristic: CornerHeuristic,
    pub lower_middles_heuristic: LowerMiddleHeuristic,
    pub upper_middles_heuristic: UpperMiddleHeuristic
}

impl Heuristic for KorfHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        let corners_distance = self.corners_heuristic.optimistic_distance_to_solved(cube);
        let lower_middles_distance = self.lower_middles_heuristic.optimistic_distance_to_solved(cube);
        let upper_middles_distance = self.upper_middles_heuristic.optimistic_distance_to_solved(cube);
        corners_distance.max(lower_middles_distance).max(upper_middles_distance)
    }
}

impl KorfHeuristic
{
    /// initialize the heuristic
    pub fn new() -> KorfHeuristic
    {
        let corners_heuristic = CornerHeuristic::new();
        let lower_middles_heuristic = LowerMiddleHeuristic::new();
        let upper_middles_heuristic = UpperMiddleHeuristic::new();
        KorfHeuristic { corners_heuristic, lower_middles_heuristic, upper_middles_heuristic }
    }
}
