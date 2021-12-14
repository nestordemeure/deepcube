use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::cube::Cube;
use super::Heuristic;

/// wrapper over heuristic to keep a count of the number of heuristic calls done
#[derive(Serialize, Deserialize)]
pub struct CounterHeuristic<H: Heuristic>
{
    /// how many times has the heuristic been called?
    count: AtomicUsize,
    /// wrapped heuristic
    #[serde(bound(deserialize = "H: DeserializeOwned"))]
    pub heuristic: H
}

impl<H: Heuristic> Heuristic for CounterHeuristic<H>
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        // increases the count
        self.count.fetch_add(1, Ordering::Relaxed);
        // does the actual heuristic computation
        self.heuristic.optimistic_distance_to_solved(cube)
    }
}

impl<H: Heuristic> CounterHeuristic<H>
{
    /// initialize the heuristic
    pub fn new(heuristic: H) -> CounterHeuristic<H>
    {
        let count = AtomicUsize::new(0);
        CounterHeuristic { heuristic, count }
    }

    /// returns the number of calls of the heuristic so far
    pub fn get_nb_calls(&self) -> usize
    {
        self.count.load(Ordering::Relaxed)
    }
}
