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
        let index = self.encoder.encode(&cube);
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
            for cube in solved_cubes.iter()
            {
                nb_new_cubes +=
                    Self::iterative_deepening(cube.clone(), &moves, &mut table, &encoder, 0, depth);
            }

            // displays the current result
            if nb_new_cubes == 0
            {
                // display final informations on the table
                timer.stop();
                println!("Corners done! (maximum distance:{} table size:{} computing time:{:?})",
                         depth,
                         table.len(),
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
    /// returns the number of new cubes found
    fn iterative_deepening(cube: Cube,
                           moves: &[Move],
                           table: &mut [Option<u8>],
                           encoder: &E,
                           depth: u8,
                           max_depth: u8)
                           -> usize
    {
        if depth == max_depth
        {
            // we are on the border, registers the cube
            let index = encoder.encode(&cube);
            match table[index]
            {
                None =>
                {
                    table[index] = Some(depth);
                    1
                }
                Some(_) => 0
            }
        }
        else
        {
            // goes further in depth
            let mut nb_new_cells = 0;
            for m in moves.iter()
            {
                let child_cube = cube.apply_move(m);
                nb_new_cells +=
                    Self::iterative_deepening(child_cube, moves, table, encoder, depth + 1, max_depth);
            }
            nb_new_cells
        }
    }
}
