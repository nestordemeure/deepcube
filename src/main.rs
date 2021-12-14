#![allow(dead_code, non_snake_case)]

mod cube;
mod solver;
pub use crate::solver::heuristic::{Heuristic, KorfHeuristic, MiddlesHeuristic, CornersHeuristic};

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
    let runtype = RunType::TestRun;

    match runtype
    {
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
            //let _corners_heuristic = CornersHeuristic::new();
            let middles_heuristic = MiddlesHeuristic::new();
            middles_heuristic.save("../data/middles_heuristic.bin");
        }
        RunType::GenerateHeuristicTables =>
        {
            // saves corners heuristics
            let corners_heuristic = CornersHeuristic::new();
            corners_heuristic.save("../data/corners_heuristic.bin");
            // saves middles heuristics
            let middles_heuristic = MiddlesHeuristic::new();
            middles_heuristic.save("../data/middles_heuristic.bin");
            // saves korf heuristics
            // built by recycling the previous two heuristics
            let korf_heuristic = KorfHeuristic { corners_heuristic, middles_heuristic };
            korf_heuristic.save("../data/korf_heuristic.bin");
        }
        RunType::SolveCube(nb_scramble) =>
        {
            // generate a scrambled cube
            let cube = cube::Cube::solved().scramble(nb_scramble);
            println!("Scrambled cube:");
            cube.display();
            // solves the cube
            let heuristic = KorfHeuristic::load("../data/korf_heuristic.bin");
            unimplemented!("no solver has been implemented yet!");
            // displays result
            println!("Solved cube:");
            cube.display();
        }
    }
}
