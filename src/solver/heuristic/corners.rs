use crate::cube::{Cube, Color, Move, NB_FACES, NB_SQUARES_CUBE, NB_SQUARES_FACE};
use super::Heuristic;
use super::super::radix_tree::{Table, CubeSet};

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
        let (corners, _middles) = cube.get_corners_middles();
        *self.table.get(&corners).expect("you submitted unknown corners!")
    }
}

impl CornersHeuristic
{
    /// takes a set of corners and turn them into a full cube
    /// all other colors are left Invalid
    fn cube_of_corners(corners: &[Color]) -> Cube
    {
        let mut squares = [Color::Invalid; NB_SQUARES_CUBE];

        for index_face in 0..NB_FACES
        {
            // extracts the face
            let start_index = index_face * NB_SQUARES_FACE;
            let end_index = start_index + NB_SQUARES_FACE;
            let face = &mut squares[start_index..end_index];
            // gets the corners corresponding to the face
            let corners_face = &corners[index_face * 4..];
            // put the four corners back in the face
            face[0] = corners_face[0];
            face[2] = corners_face[1];
            face[6] = corners_face[2];
            face[8] = corners_face[3];
        }

        Cube { squares }
    }

    pub fn new() -> CornersHeuristic
    {
        // only moves that impacts the corners
        let moves: Vec<Move> =
            Move::all_moves().into_iter().filter(|m| !m.description.kind.is_center_layer()).collect();

        // table in which we will store our results
        let mut table = Table::new();
        // set of all new cubes seen at the previous iteration
        let mut previous_cubes = CubeSet::new();
        // initialized from the solved cubes
        let mut table_size = 0;
        for cube in Cube::all_solved_cubes()
        {
            let corners = cube.get_corners_middles().0;
            table.insert(&corners, 0);
            previous_cubes.insert_key(&corners);
            table_size += 1;
        }
        let key_length = NB_FACES * 4;

        // distance from now to a solved cube
        let mut distance_to_solved: u8 = 1;
        // run until we cannot find new corners
        while !previous_cubes.is_empty()
        {
            let mut new_cubes = CubeSet::new();
            previous_cubes.for_each_key(key_length, |corners| {
                              let cube = CornersHeuristic::cube_of_corners(corners);
                              for m in moves.iter()
                              {
                                  let child = cube.apply_move(m);
                                  let corners_child = child.get_corners_middles().0;
                                  let is_new = table.insert(&corners_child, distance_to_solved);
                                  if is_new
                                  {
                                      new_cubes.insert_key(&corners_child);
                                      table_size += 1;
                                  }
                              }
                          });
            // displays information on the run so far
            println!("Did distance {} ({} distinct states so far).", distance_to_solved, table_size);
            // updates the distance and new cubes
            distance_to_solved += 1;
            previous_cubes = new_cubes;
        }

        // display final informations on the table
        println!("Done! (maximum distance:{} table size:{})", distance_to_solved - 1, table_size);
        // compresses the table and returns it
        table.compress();
        CornersHeuristic { table }
    }
}
