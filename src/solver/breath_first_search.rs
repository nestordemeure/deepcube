use std::collections::BTreeSet;
use stopwatch::Stopwatch;
use crate::cube::moves::{Move, MoveDescription};
use crate::cube::Cube;

impl Cube
{
    /// solves the given cube by breath first search
    /// (which is equivalent to Diksjtra's algrithm here)
    /// NOTE: this algorithm has the particularity of not requiring an heuristic
    /// WARNING: this algorithm can easily fill the available memory if one is not careful
    pub fn solve_breath_first_search(&self) -> Vec<MoveDescription>
    {
        // used to time the computation
        let timer = Stopwatch::start_new();
        let mut nb_cube_expanded = 0;

        // all the cubes observed so far
        let mut known_cubes = BTreeSet::new();
        known_cubes.insert(self.to_identifier());
        // current distance and cubes at the current distance
        let mut current_distance = 0;
        let mut current_cubes = vec![(self.clone(), Vec::new())];
        // all moves that can be applied to a cube
        let moves = Move::all_moves();

        // loop until we reach a result
        loop
        {
            // checks all the cubes at the current depth
            let mut new_cubes = Vec::new();
            for (cube, path) in current_cubes
            {
                // checks if we reached the target
                if cube.is_solved()
                {
                    println!("Done! Found a path of length {} in {:?} ({} cubes expanded / 0 heuristic call)",
                    current_distance,
                    timer.elapsed(),
                    nb_cube_expanded);
                    println!("Path: {:?}", path);
                    return path;
                }

                // expands the cube
                nb_cube_expanded += 1;
                for m in moves.iter()
                {
                    // applies a move
                    let child_cube = cube.apply_move(m);
                    // checks if the produced cube is new
                    let is_new_cube = known_cubes.insert(child_cube.to_identifier());
                    if is_new_cube
                    {
                        // builds the path to the child cube
                        let mut child_path = path.clone();
                        child_path.push(m.description);
                        child_path.shrink_to_fit();
                        // saves the path to the child cube
                        new_cubes.push((child_cube, child_path));
                    }
                }
            }

            // display information on the run
            println!("Breath First Search: did distance {} in {:?} ({} cubes expanded / 0 heuristic call)",
                     current_distance,
                     timer.elapsed(),
                     nb_cube_expanded);

            // updates for the next iteration
            current_cubes = new_cubes;
            current_distance += 1;
        }
    }
}
