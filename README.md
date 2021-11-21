# deepCube

An experiment in Rubik's cube solvers and in particular optimal solvers (producing a minimal path) and Deep-learning.

## Functionalities

NA

## Ideas

### Using deep learning

NA

### My own solver

NA

## TODO

one could have a single cube definition (1D)
but add a coordinate module with 1D, 2D+face and 3D+orientation coordinates; conversion functions between those and rotations on the 3D ones (that could then be used to implement rotations on the 1D ones)
the coordinate system lets us have our rotations, a flat coordinate system and easy access: the best of all worlds

- color normalization
- scrambling
- dijskra (tested on small scramble)
- heuristics
- A*
- IDA*

- some tests to check whether both representation are truly equivalent?
- some tests to check that all algorithm find optimal solutions with the same number of moves?

- implement scrambler
- implement vizualization (could be a simple shell based one)
- implement heuristics for solvers
    - cross
    - corners
    - wrapper to count number of calls to a given heuristic
- implement solvers (sequential and parallel versions):
    - greedy (?)
    - Dijskra
    - A*
    - IDA*
    - RBFS
    - my own solver
- neural network
    - collect data
    - train network
    - use it as an heuristic

## Notes:

The code requires nightly to check if vectors are sorted in debug mode.