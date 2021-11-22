use std::collections::{HashMap, hash_map::Entry};
use crate::cube::{Cube, Color, Move, NB_FACES};
use super::Heuristic;

/// estimates the number of twist needed to get the corners in the correct position
/// this is then used as a lower bound for the number of twist left before resolution
pub struct CornersHeuristic
{
    table: HashMap<[Color; NB_FACES * 4], usize>
}

impl Heuristic for CornersHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> usize
    {
        let (corners, _middles) = cube.get_corners_middles();
        *self.table.get(&corners).expect("you submitted unknown corners!")
    }
}

impl CornersHeuristic
{
    /// generates the heuristic by building the distance table
    /// WARNING: this operation is costly
    /// TODO: this generation could be made parallel
    pub fn new() -> CornersHeuristic
    {
        let moves = Move::all_moves();
        let mut cubes = vec![Cube::solved()];
        let mut table = HashMap::new();
        let mut distance_to_solved = 0;
        // iterates on incremental depth as long as we find new cubes
        while !cubes.is_empty()
        {
            let mut new_cubes = vec![];
            // iterates on all the new cubes from the latest iteration
            for cube in cubes.iter()
            {
                // applies all moves to the cube to try and generate new cubes
                for m in moves.iter()
                {
                    // applies the move to the cube to get a new cube and its hash
                    let new_cube = cube.apply_move(m);
                    let (corners, _middles) = new_cube.get_corners_middles();
                    // if the cube is truly new, saves the distance and adds it to the cube to further process
                    if let Entry::Vacant(entry) = table.entry(corners)
                    {
                        entry.insert(distance_to_solved);
                        new_cubes.push(new_cube);
                    }
                }
            }
            // displays information on the run so far
            println!("Did distance {} ({} distinct states so far, {} cubes to explore at the next iteration)",
                     distance_to_solved,
                     table.len(),
                     new_cubes.len());
            // updates the distance and ist of cubes to work on
            distance_to_solved += 1;
            cubes = new_cubes;
        }
        // display final informations on the table
        println!("Done! ({} distinct states, with a maximum distance of {})",
                 table.len(),
                 distance_to_solved - 1);
        CornersHeuristic { table }
    }
}

// TODO:
// run the search in parallel at the cubes.iter() level
// improve the hash function used to speed up computations
