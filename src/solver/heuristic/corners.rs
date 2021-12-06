use std::collections::{BTreeMap, btree_map::Entry};
use crate::cube::{Cube, Move};
use super::Heuristic;
use super::miniCube::MiniCube;

/// estimates the number of twist needed to get the corners in the correct position
/// this is then used as a lower bound for the number of twist left before resolution
pub struct CornersHeuristic
{
    table: BTreeMap<MiniCube, u8>
}

impl Heuristic for CornersHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        let mini_cube = MiniCube::from_corners(cube);
        *self.table.get(&mini_cube).expect("you submitted unknown corners!")
    }
}

impl CornersHeuristic
{
    pub fn new() -> CornersHeuristic
    {
        // set of all new cubes seen at the previous iteration
        // initialized from the solved cubes
        let mut previous_cubes: Vec<MiniCube> =
            Cube::all_solved_cubes().iter().map(MiniCube::from_corners).collect();
        // table in which we will store our results
        let mut table: BTreeMap<MiniCube, u8> =
            previous_cubes.iter().cloned().map(|mini_cube| (mini_cube, 0)).collect();
        // only moves that impacts the corners
        let moves: Vec<Move> =
            Move::all_moves().into_iter().filter(|m| !m.description.kind.is_center_layer()).collect();

        // distance from now to a solved cube
        let mut distance_to_solved: u8 = 1;
        // run until we cannot find new corners
        while !previous_cubes.is_empty()
        {
            let new_cubes = previous_cubes.into_iter()
                                          .map(MiniCube::to_corners)
                                          .flat_map(|cube| moves.iter().map(move |m| cube.apply_move(m)))
                                          .map(|cube_child| MiniCube::from_corners(&cube_child))
                                          .filter(|mini_cube_child| match table.entry(*mini_cube_child)
                                          {
                                              Entry::Vacant(entry) =>
                                              {
                                                  entry.insert(distance_to_solved);
                                                  true
                                              }
                                              _ => false
                                          })
                                          .collect();
            previous_cubes = new_cubes;
            // displays information on the run so far
            println!("Did distance {} ({} distinct states so far).", distance_to_solved, table.len());
            distance_to_solved += 1;
        }

        // display final informations on the table
        println!("Done! (maximum distance:{} table size:{})", distance_to_solved - 1, table.len());
        // returns the table
        CornersHeuristic { table }
    }
}

// measure the memory use of the cubeset and table
// if it is the cubeset that is eating at the memory, we might replace it with a vector of integers which are large enough to store the corners losslesly
// create a table type (which encapsulate a btree and the corner to int cnoversion) with new, insert and get operations
// we might want to use a more memory efficient datastructure instead of the btree: https://crates.io/crates/btree-slab
