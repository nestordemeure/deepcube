use stopwatch::Stopwatch;
use crate::cube::moves::{Move, MoveDescription, MoveKind, Amplitude};
use crate::cube::Cube;
use crate::solver::heuristic::Heuristic;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

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
                // removes the, potentially, one element too many at the end of the path
                path.truncate(target_depth);
                // displays the result
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
                // we let the path be one element longer than the length as our research will be one ahead
                while path.len() <= target_depth
                {
                    path.push(dummy_move);
                }
            }
        }
    }

    /// solves the given cube by trying depth one after the other until it finds a solved cube
    /// uses an heuristic to prune branches
    /// NOTE:
    /// - this algorithm *will* find an optimal solution but it might be slow as it will try a large number of depths
    pub fn solve_iterative_deepening_Astar_parallel<H: Heuristic + Sync>(&self,
                                                                         heuristic: &H)
                                                                         -> Vec<MoveDescription>
    {
        // used to time the computation
        let timer = Stopwatch::start_new();
        // all moves that can be applied to a cube
        let moves = Move::all_moves();
        let dummy_move = MoveDescription { kind: MoveKind::Front, amplitude: Amplitude::Clockwise };

        // tries a depth 0
        if self.is_solved()
        {
            let path = Vec::new();
            println!("Done! Found a path of length 0 in {:?} (0 cubes expanded / 0 heuristic call)",
                     timer.elapsed());
            println!("Path: {:?}", path);
            return path;
        }

        // does a single expansion and uses the result as our starting point
        let cubes_paths: Vec<(Cube, Vec<MoveDescription>)> = moves.iter()
                                                                  .map(|m| {
                                                                      let cube = self.apply_move(m);
                                                                      let path = vec![m.description];
                                                                      (cube, path)
                                                                  })
                                                                  .collect();
        let nb_cube_expanded = AtomicUsize::new(1);
        let nb_heuristic_calls = AtomicUsize::new(0);
        let mut shifted_target_depth = 0;
        loop
        {
            // tries to find a solution at the given depth
            // does it in parallel over the once expanded cubes
            let next_depth = AtomicUsize::new(usize::MAX);
            let path_option =
                cubes_paths.par_iter().find_map_any(|(cube, path)| {
                                          // increases the size of the path for the iteration
                                          // we let the path be one element longer than the length as our research will be one ahead
                                          let mut path = path.clone();
                                          while path.len() <= shifted_target_depth
                                          {
                                              path.push(dummy_move);
                                          }
                                          let cube = cube.clone();
                                          let mut next_depth_thread = usize::MAX; // upper bound on the depth of the optimal solution
                                          let mut nb_cube_expanded_thread = 0;
                                          let mut nb_heuristic_calls_thread = 0;
                                          let is_solved = Cube::solve_iterative_deepening_Astar_rec(cube,
                         &mut path,
                         &moves,
                         heuristic,
                         &mut nb_cube_expanded_thread,
                         &mut nb_heuristic_calls_thread,
                         0,
                         shifted_target_depth,
                         &mut next_depth_thread);
                                          // updates the counters
                                          next_depth.fetch_min(next_depth_thread, Ordering::Relaxed);
                                          nb_cube_expanded.fetch_add(nb_cube_expanded_thread,
                                                                     Ordering::Relaxed);
                                          nb_heuristic_calls.fetch_add(nb_heuristic_calls_thread,
                                                                       Ordering::Relaxed);
                                          // returns the result if we suceeded
                                          if is_solved
                                          {
                                              Some(path)
                                          }
                                          else
                                          {
                                              None
                                          }
                                      });
            let next_depth = next_depth.into_inner();

            // checks if we reached the target
            let target_depth = shifted_target_depth + 1;
            match path_option
            {
                Some(mut path) =>
                {
                    // removes the, potentially, one element too many at the end of the path
                    path.truncate(target_depth);
                    // displays the result
                    println!("Done! Found a path of length {} in {:?} ({} cubes expanded / {} heuristic call)",
                            target_depth, 
                            timer.elapsed(),
                            nb_cube_expanded.load(Ordering::Relaxed),
                            nb_heuristic_calls.load(Ordering::Relaxed));
                    println!("Path: {:?}", path);
                    return path;
                }
                None =>
                {
                    // display information on the run
                    println!("Iterative deepening A*: did distance {} in {:?} ({} cubes expanded / {} heuristic call)", 
                            target_depth, 
                            timer.elapsed(), 
                            nb_cube_expanded.load(Ordering::Relaxed), 
                            nb_heuristic_calls.load(Ordering::Relaxed));
                    // updates the target depth
                    if next_depth == usize::MAX
                    {
                        shifted_target_depth += 1;
                    }
                    else
                    {
                        shifted_target_depth = next_depth;
                    }
                    println!("doing depth {}", target_depth);
                }
            }
        }
    }
}
