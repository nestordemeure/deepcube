#![feature(is_sorted)]
#[allow(dead_code)]
mod cube;
mod solver;

fn main()
{
    // test the display function
    let cube = cube::Cube::solved();
    println!("Solved cube:");
    cube.display();

    // generate all moves and display them
    let moves = cube::Move::all_moves();
    for m in moves
    {
        println!("{:?}:", m.description);
        cube.apply_move(&m).display();
    }

    // generate the corner heuristic
    let corner_heuristic = solver::heuristic::CornersHeuristic::new();
}
