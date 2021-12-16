use serde::{Serialize, Deserialize};
use super::{Heuristic, UpperMiddleHeuristic, LowerMiddleHeuristic, CornerHeuristic};
use crate::cube::Cube;

/// combines the corner and middle heuristic linearly (with either an average or a sum)
/// if `USE_RAW_SUM` is set to true, the heuristic won't divide and thus will *not* be optimistic
/// making it unsuitable for algorithms such as A* and IDA*
#[derive(Serialize, Deserialize)]
pub struct RawAverageHeuristic<const USE_RAW_SUM: bool>
{
    pub corners_heuristic: CornerHeuristic,
    pub lower_middles_heuristic: LowerMiddleHeuristic,
    pub upper_middles_heuristic: UpperMiddleHeuristic
}

/// does the su of the middle and corner heuristics (rounded up)
/// this heuristic is *not* optimistic and, thus, unsuitable for algorithms such as A* and IDA*
pub type SumHeuristic = RawAverageHeuristic<true>;
/// does an average of the middle and corner heuristics (rounded up)
/// this value is not as good as a maximum but I hypothesize that it will be more information rich
pub type AverageHeuristic = RawAverageHeuristic<false>;

impl<const USE_RAW_SUM: bool> Heuristic for RawAverageHeuristic<USE_RAW_SUM>
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        // computes the individual heuristics
        let corners_distance = self.corners_heuristic.optimistic_distance_to_solved(cube);
        let lower_middles_distance = self.lower_middles_heuristic.optimistic_distance_to_solved(cube);
        let upper_middles_distance = self.upper_middles_heuristic.optimistic_distance_to_solved(cube);
        // assemble the heuristics
        let sum_distances = corners_distance + lower_middles_distance + upper_middles_distance;
        if USE_RAW_SUM
        {
            sum_distances
        }
        else
        {
            // ceil(average)
            (sum_distances + 2) / 3
        }
    }
}

impl<const USE_RAW_SUM: bool> RawAverageHeuristic<USE_RAW_SUM>
{
    /// initialize the heuristic
    pub fn new(use_raw_sum: bool) -> Self
    {
        let corners_heuristic = CornerHeuristic::new();
        let lower_middles_heuristic = LowerMiddleHeuristic::new();
        let upper_middles_heuristic = UpperMiddleHeuristic::new();
        RawAverageHeuristic { corners_heuristic, lower_middles_heuristic, upper_middles_heuristic }
    }
}
