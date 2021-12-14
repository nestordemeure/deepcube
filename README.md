# deepCube

An experiment in Rubik's cube solvers and in particular optimal solvers (solving a Rubik's cube in a minimum number of moves) and, maybe, using Deep-learning to speed up the computation *while keeping optimality proofs* valid.

## Notes

The big constraint in this problem is that due to the number of states in which a rubik's cube can be, most solvers (and most naive approach to generate table for the heuristics) *fail due to out of memory*.

The heuristic precomputations would be easy to paralelize but this increase memory use during their computation which can cause out of memory errors.

## Functionalities

- cube representation:
    - ability to display a cube
    - ability to represent various moves
    - ability to test if a cube is solved
    - ability to scramble a cube

- heuristics:
    - corners precomputed table

- solvers:

## TODO

- heuristics:
    - middles (do lower and upper)
    - use compile time known sizes for permutation computations
    - try the middles solver with 7 middles rather than 6

- solvers:
    - Best First Search (?)
    - Dijskra (tested on small scramble)
    - A*
    - IDA*
    - Recursive Best First Search
    - my own solver
        - guided by a sum of heuristics
        - guided by a weighted sum
        - guided by a neural network?
    - some tests to check that all algorithm find optimal solutions with the same number of moves?

- neural networks:
    - collect data
    - train network
    - use it as an heuristic

## References

The paper [Finding Optimal Solutions to Rubik's Cube Using Pattern Databases](https://www.cs.princeton.edu/courses/archive/fall06/cos402/papers/korfrubik.pdf) by Richard Korf is *the* reference when it comes to solve Rubik's cube optimally.
