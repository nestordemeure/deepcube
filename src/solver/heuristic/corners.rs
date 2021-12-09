use crate::cube::{Cube, Move};
use super::Heuristic;
use super::minicube::MiniCube;
use super::table::Table;

/// estimates the number of twist needed to get the corners in the correct position
/// this is then used as a lower bound for the number of twist left before resolution
pub struct CornersHeuristic
{
    table: Table
}

impl Heuristic for CornersHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        let mini_cube = MiniCube::from_corners(cube);
        self.table.get(mini_cube)
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
        let mut table: Table = previous_cubes.iter().cloned().map(|mini_cube| (mini_cube, 0)).collect();
        // only moves that impacts the corners
        let moves: Vec<Move> =
            Move::all_moves().into_iter().filter(|m| !m.description.kind.is_center_layer()).collect();

        // distance from now to a solved cube
        let mut distance_to_solved: u8 = 1;
        // run until we cannot find new corners
        while !previous_cubes.is_empty()
        {
            // turns old corners into new corners
            let new_cubes =
                previous_cubes.into_iter()
                              .map(MiniCube::to_corners)
                              .flat_map(|cube| moves.iter().map(move |m| cube.apply_move(m)))
                              .map(|cube_child| MiniCube::from_corners(&cube_child))
                              .filter(|mini_cube_child| table.insert(*mini_cube_child, distance_to_solved))
                              .collect();
            previous_cubes = new_cubes;
            // displays information on the run so far
            println!("Corners: did distance {} ({} distinct states so far).",
                     distance_to_solved,
                     table.len());
            distance_to_solved += 1;
        }

        // display final informations on the table
        // -2 as we both incremented the distance and had an iteration with no cubes: two useless iterations
        println!("Corners done! (maximum distance:{} table size:{})", distance_to_solved - 2, table.len());
        // returns the table
        CornersHeuristic { table }
    }
}
