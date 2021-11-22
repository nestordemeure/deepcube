#![feature(is_sorted)]
#[allow(dead_code)]
mod cube;
mod solver;

fn main()
{
    // generate all moves and display them
    let moves = cube::Move::all_moves();
    for m in moves
    {
        println!("{:?} => {:?}", m.description, m.permutation);
    }
    // generate the corner heuristic
    let corner_heuristic = solver::heuristic::CornersHeuristic::new();
}
