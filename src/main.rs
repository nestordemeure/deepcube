#![allow(dead_code, non_snake_case)]
mod utils;
mod cube;
mod solver;
pub use crate::solver::heuristic::{Heuristic, KorfHeuristic, LowerMiddleHeuristic, UpperMiddleHeuristic,
                               CornerHeuristic, SumHeuristic};

// sets the allocator to jemalloc
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// describes the various type of run that could happen
enum RunType
{
    /// run various test/debug operations
    TestRun,
    /// build heuristic tables for later use
    GenerateHeuristicTables,
    /// solve a random cube scrambled a given number of times
    SolveCube(usize)
}

fn main()
{
    // action to be done when running code
    //let runtype = RunType::GenerateHeuristicTables;
    let runtype = RunType::SolveCube(5);

    match runtype
    {
        RunType::SolveCube(nb_scramble) =>
        {
            // generate a scrambled cube
            let cube = cube::Cube::scrambled(nb_scramble);
            println!("Scrambled cube:");
            cube.display();

            // gets an heuristic
            //let heuristic = CornerHeuristic::load("./data/corners_heuristic.bin");
            //let heuristic = LowerMiddleHeuristic::load("./data/lower_middles_heuristic.bin");
            let heuristic = KorfHeuristic::load("./data/korf_heuristic.bin");
            //let heuristic = AverageHeuristic::load("./data/average_heuristic.bin");

            // solves the cube
            //let path = cube.solve_breath_first_search();
            //let path = cube.solve_best_first_search(&heuristic);
            //let path = cube.solve_iterative_deepening();
            //let path = cube.solve_iterative_deepening_Astar(&heuristic);
            let path = cube.solve_iterative_deepening_Astar_parallel(&heuristic);

            // displays result
            println!("Solved cube:");
            let cube = cube.apply_path(&path);
            cube.display();
        }
        RunType::GenerateHeuristicTables =>
        {
            // saves corners heuristics
            let corners_heuristic = CornerHeuristic::new();
            corners_heuristic.save("./data/corners_heuristic.bin");
            //let corners_heuristic = CornerHeuristic::load("./data/corners_heuristic.bin");
            // saves lower middles heuristics
            let lower_middles_heuristic = LowerMiddleHeuristic::new();
            lower_middles_heuristic.save("./data/lower_middles_heuristic.bin");
            //let lower_middles_heuristic = LowerMiddleHeuristic::load("./data/lower_middles_heuristic.bin");
            // saves upper middles heuristics
            let upper_middles_heuristic = UpperMiddleHeuristic::new();
            upper_middles_heuristic.save("./data/upper_middles_heuristic.bin");
            //let upper_middles_heuristic = UpperMiddleHeuristic::load("./data/upper_middles_heuristic.bin");
            // saves korf heuristics
            // built by recycling the previous two heuristics
            let korf_heuristic =
                KorfHeuristic { corners_heuristic, lower_middles_heuristic, upper_middles_heuristic };
            korf_heuristic.save("./data/korf_heuristic.bin");
            //let average_heuristic = SumHeuristic { corners_heuristic, lower_middles_heuristic, upper_middles_heuristic };
            //average_heuristic.save("./data/sum_heuristic.bin");
        }
        RunType::TestRun =>
        {
            // test the display function
            let cube = cube::Cube::solved();
            println!("Solved cube:");
            cube.display();

            /*let scrambled_cube = cube.scramble(200);
            println!("Scrambled cube:");
            scrambled_cube.display();*/

            // test all rotation of the cube
            /*for axis in cube::RotationAxis::into_enum_iter()
            {
                println!("{:?}:", axis);
                cube.rotate(axis).display();
            }*/

            // display all solved cubes
            /*for (i, cube) in cube::Cube::all_solved_cubes().iter().enumerate()
            {
                println!("solved cube {}:", i);
                cube.display();
            }*/

            // generate all moves and displays them
            /*let moves = cube::Move::all_moves();
            for m in moves
            {
                println!("{:?}:", m.description);
                cube.apply_move(&m).display();
            }*/

            // generate the heuristics
            let corners_heuristic = CornerHeuristic::new();
            corners_heuristic.save("./data/corners_heuristic.bin");
        }
    }
}
