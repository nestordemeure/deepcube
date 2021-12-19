use serde::{Serialize, Deserialize, de::DeserializeOwned};
mod encoder;
mod permutations;
use encoder::{Encoder, CornerEncoder, MiddleEncoder};
use super::Heuristic;
use crate::cube::{Cube, Move};
use progressing::{mapping::Bar, Baring};
use stopwatch::Stopwatch;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU8, AtomicI8, AtomicUsize, Ordering};

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

impl<E: Encoder + Sync> TableHeuristic<E>
{
    /// initialize the heuristic in parallel
    pub fn new() -> TableHeuristic<E>
    {
        // initializes the table and encoder
        let encoder = E::new();
        let table_size = E::nb_indexes();
        let table: Vec<AtomicU8> = (0..table_size).map(|_| AtomicU8::new(u8::MAX)).collect();

        // progress bar to track progress
        let mut progress_bar = Bar::with_range(0, table_size).timed();
        let mut timer = Stopwatch::start_new();

        // uses an iterative deepening search to fill the table
        let solved_cubes = Cube::all_solved_cubes();
        let moves = Move::all_moves();
        let mut current_table_size = 0;
        // depth left when exploring the various cubes
        let depth_cubes: Vec<AtomicI8> = (0..table_size).map(|_| AtomicI8::new(-1)).collect();
        for depth in 0..
        {
            // iterates at the given depth from all solved cubes
            let nb_new_cubes = AtomicUsize::new(0);
            solved_cubes.par_iter().for_each(|cube| {
                                       let mut nb_new_cubes_thread = 0;
                                       Self::iterative_deepening(cube,
                                                                 &moves,
                                                                 &depth_cubes,
                                                                 &table,
                                                                 &mut nb_new_cubes_thread,
                                                                 &encoder,
                                                                 0,
                                                                 depth);
                                       nb_new_cubes.fetch_add(nb_new_cubes_thread, Ordering::Relaxed);
                                   });
            let nb_new_cubes = nb_new_cubes.into_inner();

            // take into account the fact that the table size might be approximative
            current_table_size += nb_new_cubes;
            let stopping_condition = (current_table_size >= table_size)
                                     && table.par_iter()
                                             .all(|distance| distance.load(Ordering::Relaxed) < u8::MAX);
            // displays the current result
            if stopping_condition
            {
                // display final informations on the table
                timer.stop();
                println!("Table done! (maximum distance:{} table size:{} computing time:{:?})",
                         depth,
                         table_size,
                         timer.elapsed());
                break;
            }
            else
            {
                // displays information on the current depth
                progress_bar.set(current_table_size);
                println!("Table: did distance {} {} in {:?}", depth, progress_bar, timer.elapsed());
            }
        }

        // removes the options
        let table: Vec<u8> =
            table.into_par_iter().map(|atomic_distance| atomic_distance.into_inner()).collect();
        TableHeuristic { encoder, table }
    }

    /// registers all new cubes at depth max_depth
    fn iterative_deepening(cube: &Cube,
                           moves: &[Move],
                           depth_cubes: &[AtomicI8],
                           table: &[AtomicU8],
                           nb_new_cubes: &mut usize,
                           encoder: &E,
                           depth: u8,
                           max_depth: u8)
    {
        // avoids running code on cubes whose children are all known
        let index = encoder.encode(cube);
        let depth_left = (max_depth - depth) as i8;
        if depth_cubes[index].load(Ordering::Relaxed) < depth_left
        {
            // registers cube
            // NOTE: the load then store is not atomic so we might get into this branch by error
            // this does not impact the final table (all thread are at equal depth)
            // but the number of new cubes might be artificially raised because of such errors
            depth_cubes[index].store(depth_left, Ordering::Relaxed);

            if depth == max_depth
            {
                // we are at the depth limit, registers the depth
                if table[index].load(Ordering::Relaxed) == u8::MAX
                {
                    table[index].store(depth, Ordering::Relaxed);
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
                                              depth_cubes,
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

impl<E: Encoder + Sync> Default for TableHeuristic<E>
{
    fn default() -> Self
    {
        Self::new()
    }
}
