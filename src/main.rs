#![feature(is_sorted)]
#[allow(dead_code)]
mod cube;
mod solver;

fn main()
{
    // generate the ocrner heuristic
    let corner_heuristic = solver::heuristic::CornersHeuristic::new();
}
