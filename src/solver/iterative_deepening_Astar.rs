use stopwatch::Stopwatch;
use crate::cube::moves::{Move, MoveDescription, MoveKind, Amplitude};
use crate::cube::Cube;
use crate::solver::heuristic::Heuristic;

impl Cube
{
    /// returns true if it finds a solution at depth `target_depth`
    /// path will then contain the path to the solution
    fn solve_iterative_deepening_Astar_rec<H: Heuristic>(cube: Cube,
                                                         path: &mut [MoveDescription],
                                                         moves: &[Move],
                                                         heuristic: &H,
                                                         nb_cube_expanded: &mut usize,
                                                         nb_heuristic_calls: &mut usize,
                                                         depth: usize,
                                                         target_depth: usize,
                                                         next_depth: &mut usize)
                                                         -> bool
    {
        // lower bound on the number of steps needed to do a solve with this move
        *nb_heuristic_calls += 1;
        let minimum_final_depth = depth + heuristic.optimistic_distance_to_solved(&cube) as usize;
        //println!("depth:{} min:{} total:{}", depth, minimum_final_depth - depth, minimum_final_depth);
        match minimum_final_depth.cmp(&target_depth)
        {
            std::cmp::Ordering::Greater =>
            {
                // minimum_final_depth > target_depth
                // updates the depth for the next round
                if minimum_final_depth < *next_depth
                {
                    *next_depth = minimum_final_depth;
                }
                false
            }
            std::cmp::Ordering::Equal if cube.is_solved() =>
            {
                // minimum_final_depth == target_depth
                // we found a solution
                true
            }
            _ =>
            {
                // minimum_final_depth <= target_depth
                // expands to the next depth
                *nb_cube_expanded += 1;
                for m in moves.iter()
                {
                    // applies a move
                    let child_cube = cube.apply_move(m);
                    // updates the path
                    path[depth] = m.description;
                    // goes one depth further
                    let is_sucess = Cube::solve_iterative_deepening_Astar_rec(child_cube,
                                                                              path,
                                                                              moves,
                                                                              heuristic,
                                                                              nb_cube_expanded,
                                                                              nb_heuristic_calls,
                                                                              depth + 1,
                                                                              target_depth,
                                                                              next_depth);
                    if is_sucess
                    {
                        return true;
                    }
                }
                // we did not find a solution at the given depth
                false
            }
        }
    }

    /// solves the given cube by trying depth one after the other until it finds a solved cube
    /// uses an heuristic to prune branches
    /// NOTE:
    /// - this algorithm *will* find an optimal solution but it might be slow as it will try a large number of depths
    pub fn solve_iterative_deepening_Astar<H: Heuristic>(&self, heuristic: &H) -> Vec<MoveDescription>
    {
        // used to time the computation
        let timer = Stopwatch::start_new();
        let mut nb_cube_expanded = 0;
        let mut nb_heuristic_calls = 0;
        // all moves that can be applied to a cube
        let moves = Move::all_moves();
        let dummy_move = MoveDescription { kind: MoveKind::Front, amplitude: Amplitude::Clockwise };

        println!("heuristic: {}", heuristic.optimistic_distance_to_solved(self));

        let mut target_depth = 0;
        let mut path = Vec::new();
        loop
        {
            // tries to find a solution at the given depth
            let cube = self.clone();
            let mut next_depth = usize::MAX; // upper bound on the depth of the optimal solution
            let is_solved = Cube::solve_iterative_deepening_Astar_rec(cube,
                                                                      &mut path,
                                                                      &moves,
                                                                      heuristic,
                                                                      &mut nb_cube_expanded,
                                                                      &mut nb_heuristic_calls,
                                                                      0,
                                                                      target_depth,
                                                                      &mut next_depth);

            // checks if we reached the target
            if is_solved
            {
                println!("Done! Found a path of length {} in {:?} ({} cubes expanded / {} heuristic call)",
                         target_depth,
                         timer.elapsed(),
                         nb_cube_expanded,
                         nb_heuristic_calls);
                println!("Path: {:?}", path);
                return path;
            }
            else
            {
                // display information on the run
                println!("Iterative deepening A*: did distance {} in {:?} ({} cubes expanded / {} heuristic call)", target_depth, timer.elapsed(), nb_cube_expanded, nb_heuristic_calls);
                // updates the target depth
                if next_depth == usize::MAX
                {
                    target_depth += 1;
                }
                else
                {
                    target_depth = next_depth;
                }
                println!("doing depth {}", target_depth);
                // increases the size of the path for the next iteration
                while path.len() < target_depth
                {
                    path.push(dummy_move);
                }
            }
        }
        panic!()
    }
}
