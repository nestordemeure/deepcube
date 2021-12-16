use crate::cube::Cube;
// heuristics
mod table;
pub use table::{CornerHeuristic, LowerMiddleHeuristic, UpperMiddleHeuristic};
mod korf;
pub use korf::KorfHeuristic;
mod average;
pub use average::{AverageHeuristic, SumHeuristic};
mod counter;
pub use counter::CounterHeuristic;
// for serialization
use std::fs::File;
use std::io::{BufWriter, BufReader};
use serde::{Serialize, de::DeserializeOwned};
use bincode::{serialize_into, deserialize_from};

/// implemented by all heuristics to be used in algorithms such as A*
pub trait Heuristic: Serialize + DeserializeOwned + Sized
{
    /// returns a lower bound on the number of move that will have to be applied before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8;

    /// save the heuristic to the given file
    fn save(&self, file_name: &str)
    {
        let mut file = BufWriter::new(File::create(file_name).expect("save: unable to create the file"));
        serialize_into(&mut file, self).expect("save: unable to serialize");
    }

    /// loads the heuristic from the given file
    fn load(file_name: &str) -> Self
    {
        let mut file = BufReader::new(File::open(file_name).expect("load: unable to create the file"));
        deserialize_from(&mut file).expect("load: unable to deserialize")
    }

    /// wraps the heuristic with a counter so that we can keep track of the number of heuristic calls
    fn counter(self) -> CounterHeuristic<Self>
    {
        CounterHeuristic::new(self)
    }
}
