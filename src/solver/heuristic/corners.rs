use std::collections::{BTreeMap};
use crate::cube::{Cube, Color, Move, NB_FACES, NB_SQUARES_CUBE, NB_SQUARES_FACE};
use super::Heuristic;

/// estimates the number of twist needed to get the corners in the correct position
/// this is then used as a lower bound for the number of twist left before resolution
pub struct CornersHeuristic
{
    table: BTreeMap<[Color; NB_FACES * 4], u8>
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
    fn cube_of_corners(corners: &[Color; NB_FACES * 4]) -> Cube
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
        // initialized from the solved cubes
        let mut table: BTreeMap<[Color; NB_FACES * 4], u8> =
            Cube::all_solved_cubes().into_iter()
                                    .map(|cube| cube.get_corners_middles().0)
                                    .map(|corners| (corners, 0))
                                    .collect();
        // distance from now to a solved cube
        let mut distance_to_solved: u8 = 1;
        // run until we cannot find new corners
        loop
        {
            let mut new_corners = Vec::new();
            for (corners, distance) in table.iter()
            {
                if *distance == distance_to_solved - 1
                {
                    let cube = CornersHeuristic::cube_of_corners(corners);
                    for m in moves.iter()
                    {
                        let child = cube.apply_move(m);
                        let corners_child = child.get_corners_middles().0;
                        if !table.contains_key(&corners_child)
                        {
                            new_corners.push(corners_child);
                        }
                    }
                }
            }
            // the loop finishes when no more new corners can be found
            if new_corners.is_empty()
            {
                break;
            }
            else
            {
                // add the new corners to the table
                for corners in new_corners
                {
                    table.insert(corners, distance_to_solved);
                }
                // displays information on the run so far
                println!("Did distance {} ({} distinct states so far)", distance_to_solved, table.len());
                // updates the distance
                distance_to_solved += 1;
            }
        }

        // display final informations on the table
        println!("Done! ({} distinct states, with a maximum distance of {})",
                 table.len(),
                 distance_to_solved - 1);
        CornersHeuristic { table: table.into_iter().collect() }
    }
}

/*
there would be a trieMap with *all* the values found so far and the associated distances
and a treeSet with only the new ones (we never store in a vector as it is a memory hungry representation)

we iterate on the trie of new values
for each of them we produce childrens
we try to insert the children in the trie of all the values (keeping only the children that are new)
then we collect them into a new trie of new values
*/
