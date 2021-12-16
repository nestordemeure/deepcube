use serde::{Serialize, de::DeserializeOwned};
use crate::cube::Cube;
mod corners;
mod middles;
pub use corners::CornerEncoder;
pub use middles::MiddleEncoder;

/// used to turn a cube into an index into an array
/// garanties that the index will be continuous in memory
pub trait Encoder: Serialize + DeserializeOwned + Sized
{
    /// initializes the encoder
    fn new() -> Self;

    /// size of the array in which to put the indexes
    fn nb_indexes() -> usize;

    /// encodes a cube as an index
    fn encode(&self, cube: &Cube) -> usize;
}
