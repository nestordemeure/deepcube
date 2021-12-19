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
    - middles precomputed table
    - korf heuristic (corners plus middles)
    - ability to save and load precomputed tables on disk
    - ability to wrap heuristics to count the number of heuristic calls

- solvers:
    - best first search
    - breath first search
    - iterative deepening
    - IDA*

## TODO

- cube:
    - add function producing the worst cube possible (solve in 20 moves)

- heuristics:
    - take corner and middle heuristic as input for larger ones
    - introduce operation to end early when there is a lower bound?
    - neural networks:
        - collect data
        - train network
        - use it as an imperfect heuristic

- solvers:
    - IDA*:
        - deal with fact that length of path might be longer than actual length of solution
    - best first search
        - introduce random tie-break in minimum to avoid loops?
    - breath first search
        - modify vector in place to decrease memory use
        - check for solve *when creating a child* in order to gain one level of depth with no additional memory use
    - A*
    - Recursive Best First Search
    - my own solver
        - guided by a sum of heuristics
        - guided by a weighted sum
        - guided by a neural network?
    - some tests to check that all algorithm find optimal solutions with the same number of moves?

## References

The paper [Finding Optimal Solutions to Rubik's Cube Using Pattern Databases](https://www.cs.princeton.edu/courses/archive/fall06/cos402/papers/korfrubik.pdf) by Richard Korf is *the* reference when it comes to solve Rubik's cube optimally.

[deepCubeA](https://github.com/forestagostinelli/DeepCubeA) is a machine learning project using deep learnign and reinforcement learning to design a neural network able to solve a Rubik's cube.
However, while their network manages to solve 100% of Rubik's cube they tested, they do not aim for optimal solutions (they found that about 60% of the solution suggested by the network were optimal).

There is another interesting Rubik's cube implementation in rust, [Rusty-Rubik](https://github.com/esqu1/Rusty-Rubik).
They represent the cube in term of permutations and orientations of corners and middles pieces (instead of a flat representation as we do here) which makes computing the heuristics much more efficient.
Furthermore, they use slightly different heuristics (decoupling the permutation and orientation of middles which sounds like a good idea to do all middles at once).
However, they implement less solvers.

You can find a good Rubik's cube online simulator [here](https://ruwix.com/online-puzzle-simulators/).
