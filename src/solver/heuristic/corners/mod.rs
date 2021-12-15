use serde::{Serialize, Deserialize};
use progressing::{clamping::Bar, Baring};
use stopwatch::Stopwatch;
mod encoding;
use crate::cube::{Cube, Move};
use super::Heuristic;
use encoding::CornerEncoder;
use crate::utils::optionu8::OptionU8;

/// estimates the number of twist needed to get the corners in the correct position
/// this is then used as a lower bound for the number of twist left before resolution
#[derive(Serialize, Deserialize)]
pub struct CornersHeuristic
{
    encoder: CornerEncoder,
    table: Vec<u8>
}

impl Heuristic for CornersHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        let index = self.encoder.corners_code_of_cube(cube);
        self.table[index]
    }
}

impl CornersHeuristic
{
    /// initialize the heuristic
    pub fn new() -> CornersHeuristic
    {
        // builds a new encoder
        let encoder = CornerEncoder::new();
        let table_size = encoder.nb_corners_code();
        assert!(u32::try_from(table_size).is_ok(), "cannot fit table indices in 32 bits");

        // progress bar to track progress
        let mut progress_bar = Bar::new();
        let mut timer = Stopwatch::start_new();

        // builds a new table, full of None so far
        let mut table: Vec<OptionU8> = (0..table_size).map(|_| OptionU8::none()).collect();
        let mut nb_states = 0;

        // set of all new cubes seen at the previous iteration
        // initialized from the solved cubes
        let mut previous_cubes: Vec<u32> =
            Cube::all_solved_cubes().iter().map(|cube| encoder.corners_code_of_cube(cube) as u32).collect();
        // stores the first generation of results in the table
        previous_cubes.iter().for_each(|index| {
                                 table[*index as usize].set(0);
                             });
        nb_states += previous_cubes.len();

        // only moves that impacts the corners
        let moves: Vec<Move> =
            Move::all_moves().into_iter().filter(|m| !m.description.kind.is_center_layer()).collect();

        // distance from now to a solved cube
        let mut distance_to_solved: u8 = 1;
        // run until we cannot find new corners
        while !previous_cubes.is_empty()
        {
            // turns old corners into new corners
            /*let new_cubes = previous_cubes.into_iter()
            .map(|code| encoder.cube_of_corner_code(code as usize))
            .flat_map(|cube| moves.iter().map(move |m| cube.apply_move(m)))
            .map(|cube_child| encoder.corners_code_of_cube(&cube_child))
            .filter(|code_child| table[*code_child].set(distance_to_solved))
            .map(|code_child| code_child as u32)
            .collect();
            previous_cubes = new_cubes;*/
            // the loop is done in place to minimize memory use
            let mut i = 0;
            let mut nb_old_cubes = previous_cubes.len();
            let mut nb_new_cubes = 0;
            while i < nb_old_cubes
            {
                // gets the current cube
                let code = previous_cubes[i];
                let cube = encoder.cube_of_corner_code(code as usize);
                // pushes its child at the end of the vector
                for cube_child in moves.iter().map(move |m| cube.apply_move(m))
                {
                    let code_child = encoder.corners_code_of_cube(&cube_child);
                    if table[code_child].set(distance_to_solved)
                    {
                        previous_cubes.push(code_child as u32);
                        nb_new_cubes += 1;
                    }
                }
                // removes the current cube
                previous_cubes.swap_remove(i);
                // udpate the indices
                if nb_new_cubes > 0
                {
                    // if the cube was replaced by a new cube, we can go forward into older cubes
                    nb_new_cubes -= 1;
                    i += 1;
                }
                else
                {
                    // if it wasn't, we stay in place and read a previous cube
                    nb_old_cubes -= 1;
                }
            }
            // displays information on the run so far
            nb_states += previous_cubes.len();
            let progress = (nb_states * 2 - previous_cubes.len()) as f64 / (table_size * 2) as f64;
            progress_bar.set(progress);
            println!("Corners: did distance {} {} {}/{} states in {:?}",
                     distance_to_solved,
                     progress_bar,
                     nb_states,
                     table_size,
                     timer.elapsed());
            distance_to_solved += 1;
        }

        // display final informations on the table
        // -2 as we both incremented the distance and had an iteration with no cubes: two useless iterations
        timer.stop();
        println!("Corners done! (maximum distance:{} table size:{} computing time:{:?})",
                 distance_to_solved - 2,
                 table.len(),
                 timer.elapsed());

        // turns options into values now that the full table should be filled
        let table = table.into_iter().map(|d| d.unwrap()).collect();
        // returns the table
        CornersHeuristic { encoder, table }
    }
}
