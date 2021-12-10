mod encoding;
use crate::cube::{Cube, Move};
use super::Heuristic;
use encoding::MiddleEncoder;

/// estimates the number of twist needed to get the Middles in the correct position
/// this is then used as a lower bound for the number of twist left before resolution
pub struct MiddlesHeuristic
{
    encoder: MiddleEncoder,
    table: Vec<u8>
}

impl Heuristic for MiddlesHeuristic
{
    /// returns a lower bound on the number of steps before the problem will be solved
    fn optimistic_distance_to_solved(&self, cube: &Cube) -> u8
    {
        let index = self.encoder.middles_code_of_cube(cube);
        self.table[index]
    }
}

impl MiddlesHeuristic
{
    /// initialize the heuristic
    pub fn new() -> MiddlesHeuristic
    {
        // builds a new encoder
        let encoder = MiddleEncoder::new();

        // builds a new table, full of None so far
        let mut table: Vec<Option<u8>> = (0..encoder.nb_middles_code()).map(|_| None).collect();
        let mut nb_states = 0;

        // set of all new cubes seen at the previous iteration
        // initialized from the solved cubes
        let mut previous_cubes: Vec<usize> =
            Cube::all_solved_cubes().iter().map(|cube| encoder.middles_code_of_cube(cube)).collect();
        // stores the first generation of results in the table
        previous_cubes.iter().for_each(|index| table[*index] = Some(0));
        nb_states += previous_cubes.len();

        // all moves
        let moves: Vec<Move> = Move::all_moves();

        // distance from now to a solved cube
        let mut distance_to_solved: u8 = 1;
        // run until we cannot find new Middles
        while !previous_cubes.is_empty()
        {
            // turns old Middles into new Middles
            let new_cubes = previous_cubes.into_iter()
                                          .map(|code| encoder.cube_of_middle_code(code))
                                          .flat_map(|cube| moves.iter().map(move |m| cube.apply_move(m)))
                                          .map(|cube_child| encoder.middles_code_of_cube(&cube_child))
                                          .filter(|code_child| {
                                              let result = &mut table[*code_child];
                                              match result
                                              {
                                                  None =>
                                                  {
                                                      *result = Some(distance_to_solved);
                                                      true
                                                  }
                                                  Some(_) => false
                                              }
                                          })
                                          .collect();
            previous_cubes = new_cubes;
            nb_states += previous_cubes.len();
            // displays information on the run so far
            println!("Middles: did distance {} ({} distinct states so far).", distance_to_solved, nb_states);
            distance_to_solved += 1;
        }

        // display final informations on the table
        // -2 as we both incremented the distance and had an iteration with no cubes: two useless iterations
        println!("Middles done! (maximum distance:{} table size:{})", distance_to_solved - 2, table.len());

        // turns options into values now that the full table should be filled
        let table = table.into_iter().map(|d| d.expect("this code has not been encountered!")).collect();
        // returns the table
        MiddlesHeuristic { encoder, table }
    }
}
