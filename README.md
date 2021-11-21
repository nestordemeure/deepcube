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