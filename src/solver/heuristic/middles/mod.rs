use serde::{Serialize, Deserialize};
use progressing::{clamping::Bar, Baring};
use stopwatch::Stopwatch;
mod encoding;
use crate::cube::{Cube, Move};
use super::Heuristic;
use encoding::MiddleEncoder;
use crate::utils::optionu8::OptionU8;

/// estimates the number of twist needed to get the Middles in the correct position
/// this is then used as a lower bound for the number of twist left before resolution
#[derive(Serialize, Deserialize)]
pub struct MiddlesHeuristic
{
    /// use to encode a cube into some table index
    encoder: MiddleEncoder,
    /// table for the lower middle indices
    table_upper: Vec<u8>,
    /// table for the upper middle indices
    table_lower: Vec<u8>
}

impl Heuristic for MiddlesHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        let distance_upper = self.optimistic_distance_to_solved_upper(cube);
        let distance_lower = self.optimistic_distance_to_solved_lower(cube);
        distance_lower.max(distance_upper)
    }
}

impl MiddlesHeuristic
{
    /// initialize the heuristic
    pub fn new() -> MiddlesHeuristic
    {
        let encoder = MiddleEncoder::new();
        let table_upper = Self::compute_lower_table(&encoder, false);
        let table_lower = Self::compute_lower_table(&encoder, true);
        MiddlesHeuristic { encoder, table_upper, table_lower }
    }

    /// returns a lower bound on the number of steps before the problem will be solved
    /// using the lower table
    pub fn optimistic_distance_to_solved_lower(&self, cube: &Cube) -> u8
    {
        let index = self.encoder.middles_code_of_cube(cube, true);
        self.table_lower[index]
    }

    /// returns a lower bound on the number of steps before the problem will be solved
    /// using the upper table
    pub fn optimistic_distance_to_solved_upper(&self, cube: &Cube) -> u8
    {
        let index = self.encoder.middles_code_of_cube(cube, false);
        self.table_upper[index]
    }

    /// initialize the table (lower or upper)
    fn compute_lower_table(encoder: &MiddleEncoder, use_lower_middles: bool) -> Vec<u8>
    {
        // progress bar to track progress
        let mut progress_bar = Bar::new();
        let mut timer = Stopwatch::start_new();

        // builds a new table, full of None so far
        let table_size = encoder.nb_middles_code();
        assert!(u32::try_from(table_size).is_ok(), "cannot fit table indices in 32 bits");
        let mut table: Vec<OptionU8> = (0..table_size).map(|_| OptionU8::none()).collect();
        let mut nb_states = 0;

        // set of all new cubes seen at the previous iteration
        // initialized from the solved cubes
        let mut previous_cubes: Vec<u32> =
            Cube::all_solved_cubes().iter()
                                    .map(|cube| encoder.middles_code_of_cube(cube, use_lower_middles) as u32)
                                    .collect();
        // stores the first generation of results in the table
        previous_cubes.iter().for_each(|index| {
                                 table[*index as usize].set(0);
                             });
        nb_states += previous_cubes.len();

        // all moves
        let moves: Vec<Move> = Move::all_moves();

        // distance from now to a solved cube
        let mut distance_to_solved: u8 = 1;
        // run until we cannot find new Middles
        while !previous_cubes.is_empty()
        {
            // turns old middles into new middles
            /*let new_cubes =
                previous_cubes.into_iter()
                              .map(|code| encoder.cube_of_middle_code(code as usize, use_lower_middles))
                              .flat_map(|cube| moves.iter().map(move |m| cube.apply_move(m)))
                              .map(|cube_child| encoder.middles_code_of_cube(&cube_child, use_lower_middles))
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
                let cube = encoder.cube_of_middle_code(code as usize, use_lower_middles);
                // pushes its child at the end of the vector
                for cube_child in moves.iter().map(move |m| cube.apply_move(m))
                {
                    let code_child = encoder.middles_code_of_cube(&cube_child, use_lower_middles);
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
            println!("Middles: did distance {} {} {}/{} states in {:?}",
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
        println!("Middles done! (maximum distance:{} table size:{} computing time:{:?})",
                 distance_to_solved - 2,
                 table.len(),
                 timer.elapsed());

        // turns options into values now that the full table should be filled
        table.into_iter().map(|d| d.unwrap()).collect()
    }
}
