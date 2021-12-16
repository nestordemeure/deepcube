use serde::{Serialize, Deserialize, de::DeserializeOwned};
mod encoder;
mod permutations;
use encoder::{Encoder, CornerEncoder, MiddleEncoder};
use super::Heuristic;
use crate::cube::{Cube, Move};
use progressing::{mapping::Bar, Baring};
use stopwatch::Stopwatch;

// some common heuristics
pub type CornerHeuristic = TableHeuristic<CornerEncoder>;
pub type LowerMiddleHeuristic = TableHeuristic<MiddleEncoder<true>>;
pub type UpperMiddleHeuristic = TableHeuristic<MiddleEncoder<false>>;

#[derive(Serialize, Deserialize)]
pub struct TableHeuristic<E: Encoder>
{
    #[serde(bound(deserialize = "E: DeserializeOwned"))]
    encoder: E,
    table: Vec<u8>
}

impl<E: Encoder> Heuristic for TableHeuristic<E>
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        let index = self.encoder.encode(cube);
        self.table[index]
    }
}

impl<E: Encoder> TableHeuristic<E>
{
    /// initialize the heuristic
    pub fn new() -> TableHeuristic<E>
    {
        // initializes the table and encoder
        let encoder = E::new();
        let table_size = E::nb_indexes();
        let mut table = vec![None; table_size];

        // progress bar to track progress
        let mut progress_bar = Bar::with_range(0, table_size);
        let mut timer = Stopwatch::start_new();

        // uses an iterative deepening search to fill the table
        let solved_cubes = Cube::all_solved_cubes();
        let moves = Move::all_moves();
        let mut current_table_size = 0;
        for depth in 0..
        {
            // iterates at the given depth from all solved cubes
            let mut nb_new_cubes = 0;
            let mut known_cubes = vec![false; table_size];
            for cube in solved_cubes.iter()
            {
                Self::iterative_deepening(cube,
                                          &moves,
                                          &mut known_cubes,
                                          &mut table,
                                          &mut nb_new_cubes,
                                          &encoder,
                                          0,
                                          depth);
            }

            // displays the current result
            if nb_new_cubes == 0
            {
                // display final informations on the table
                timer.stop();
                println!("Corners done! (maximum distance:{} table size:{} computing time:{:?})",
                         depth,
                         table_size,
                         timer.elapsed());
                break;
            }
            else
            {
                // displays information on the current depth
                current_table_size += nb_new_cubes;
                progress_bar.set(current_table_size);
                println!("Table: did distance {} {} {}/{} states in {:?}",
                         depth,
                         progress_bar,
                         current_table_size,
                         table_size,
                         timer.elapsed());
            }
        }

        // removes the options
        let table = table.into_iter()
                         .map(|distance_option| distance_option.expect("index was never encountered!"))
                         .collect();
        TableHeuristic { encoder, table }
    }

    /// registers all new cubes at depth max_depth
    fn iterative_deepening(cube: &Cube,
                           moves: &[Move],
                           known_cubes: &mut [bool],
                           table: &mut [Option<u8>],
                           nb_new_cubes: &mut usize,
                           encoder: &E,
                           depth: u8,
                           max_depth: u8)
    {
        // avoids running code on known cubes
        let index = encoder.encode(cube);
        let is_known = known_cubes[index];
        if !is_known
        {
            // registers cube
            known_cubes[index] = true;

            if depth == max_depth
            {
                // we are at the depth limit, registers the cube
                if table[index].is_none()
                {
                    table[index] = Some(depth);
                    *nb_new_cubes += 1;
                }
            }
            else
            {
                // goes further in depth
                for m in moves.iter()
                {
                    let child_cube = cube.apply_move(m);
                    Self::iterative_deepening(&child_cube,
                                              moves,
                                              known_cubes,
                                              table,
                                              nb_new_cubes,
                                              encoder,
                                              depth + 1,
                                              max_depth);
                }
            }
        }
    }
}

impl<E: Encoder> Default for TableHeuristic<E>
{
    fn default() -> Self
    {
        Self::new()
    }
}

/*
the known cube mecanism results in having less cubes per depth that we should
also that number *changes* from one iteration to the next
one explaination could be non-injectivity or the encoder function (it does not explain the non determinism)
*/
