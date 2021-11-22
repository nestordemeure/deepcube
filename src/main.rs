#![feature(is_sorted)]

use enum_iterator::IntoEnumIterator;
#[allow(dead_code)]
mod cube;
mod solver;

fn main()
{
    // test the display function
    let cube = cube::Cube::solved();
    println!("Solved cube:");
    cube.display();

    // test all rotation of the cube
    /*for axis in cube::RotationAxis::into_enum_iter()
    {
        println!("{:?}:", axis);
        cube.rotate(axis).display();
    }*/

    // generate all moves and displays them
    /*let moves = cube::Move::all_moves();
    for m in moves
    {
        println!("{:?}:", m.description);
        cube.apply_move(&m).display();
    }*/

    // generate the corner heuristic
    //let corner_heuristic = solver::heuristic::CornersHeuristic::new();
}
