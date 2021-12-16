use stopwatch::Stopwatch;
use crate::cube::moves::{Move, MoveDescription};
use crate::cube::Cube;
use crate::solver::heuristic::Heuristic;

impl Cube
{
    /// solves the given cube by using the most promising move greedily
    /// NOTE:
    /// - this algorithm uses O(1) memory
    /// WARNINGS:
    /// - this algorithm might never find a solution
    /// - this algorithm might find a non-optimal solution
    pub fn solve_best_first_search<H: Heuristic>(&self, heuristic: &H) -> Vec<MoveDescription>
    {
        // used to time the computation
        let timer = Stopwatch::start_new();
        let mut nb_cube_expanded = 0;
        let mut nb_heuristic_calls = 0;
        // all moves that can be applied to a cube
        let moves = Move::all_moves();
        // the final result
        let mut depth = 0;
        let mut path = Vec::new();
        // the current cube
        let mut cube = self.clone();

        while !cube.is_solved()
        {
            // display information on the run
            println!("Best first search: did distance {} in {:?} ({} cubes expanded / {} heuristic call)",
                     depth,
                     timer.elapsed(),
                     nb_cube_expanded,
                     nb_heuristic_calls);
            // finds the best child according to the heuristic
            let (child, description) = moves.iter()
                                            .map(|m| (cube.apply_move(m), m.description))
                                            .min_by_key(|(c, _d)| heuristic.optimistic_distance_to_solved(c))
                                            .unwrap();
            // updates information
            cube = child;
            path.push(description);
            depth += 1;
            nb_cube_expanded += 1;
            nb_heuristic_calls += moves.len();
        }

        // displays information on the result
        println!("Done! Found a path of length {} in {:?} ({} cubes expanded / {} heuristic call)",
                 depth,
                 timer.elapsed(),
                 nb_cube_expanded,
                 nb_heuristic_calls);
        println!("Path: {:?}", path);
        path
    }
}
