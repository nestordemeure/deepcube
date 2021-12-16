use stopwatch::Stopwatch;
use crate::cube::moves::{Move, MoveDescription, MoveKind, Amplitude};
use crate::cube::Cube;

impl Cube
{
    /// returns true if it finds a solution at depth `target_depth`
    /// path will then contain the path to the solution
    fn solve_iterative_deepening_rec(cube: Cube,
                                     path: &mut [MoveDescription],
                                     moves: &[Move],
                                     nb_cube_expanded: &mut usize,
                                     depth: usize,
                                     target_depth: usize)
                                     -> bool
    {
        if depth >= target_depth
        {
            // check if the cube is a solution
            cube.is_solved()
        }
        else
        {
            // expands to the next depth
            *nb_cube_expanded += 1;
            for m in moves.iter()
            {
                // applies a move
                let child_cube = cube.apply_move(m);
                // updates the path
                path[depth] = m.description;
                // goes one depth further
                let is_sucess = Cube::solve_iterative_deepening_rec(child_cube,
                                                                    path,
                                                                    moves,
                                                                    nb_cube_expanded,
                                                                    depth + 1,
                                                                    target_depth);
                if is_sucess
                {
                    return true;
                }
            }
            // we did not find a solution at the given depth
            false
        }
    }

    /// solves the given cube by trying depth one after the other until it finds a solved cube
    /// NOTE:
    /// - this algorithm has the particularity of not requiring an heuristic
    /// - this algorithm *will* find an optimal solution but might be significantly slow
    ///   as it iteraterates on all possible cubes by increasing depth
    pub fn solve_iterative_deepening(&self) -> Vec<MoveDescription>
    {
        // used to time the computation
        let timer = Stopwatch::start_new();
        let mut nb_cube_expanded = 0;
        // all moves that can be applied to a cube
        let moves = Move::all_moves();
        let dummy_move = MoveDescription { kind: MoveKind::Front, amplitude: Amplitude::Clockwise };

        let mut path = Vec::new();
        for target_depth in 0..
        {
            // tries to find a solution at the given depth
            let cube = self.clone();
            let is_solved = Cube::solve_iterative_deepening_rec(cube,
                                                                &mut path,
                                                                &moves,
                                                                &mut nb_cube_expanded,
                                                                0,
                                                                target_depth);

            // checks if we reached the target
            if is_solved
            {
                println!("Done! Found a path of length {} in {:?} ({} cubes expanded / 0 heuristic call)",
                         target_depth,
                         timer.elapsed(),
                         nb_cube_expanded);
                println!("Path: {:?}", path);
                return path;
            }
            else
            {
                // display information on the run
                println!("Iterative deepening: did distance {} in {:?} ({} cubes expanded / 0 heuristic call)", target_depth, timer.elapsed(), nb_cube_expanded);
                // increases the size of the path for the next iteration
                path.push(dummy_move);
            }
        }
        // notes that there is a solution
        unreachable!("Either a solution exists or the solver will run forever.")
    }
}
