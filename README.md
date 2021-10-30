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

a move is build from a simple description
it contains the description (which can be used as an identifier and should the thing displayed when displaying)
and a permutation table used when actually aplying the move

moves have a new funtion, an apply function and a compose function
they can also be created in an orientation preserving new_orientation_preserving

some research on move notation and face numbering might be good

moves could be expressed in very simple components (quater turn top/center/bottom) that are then composed (x1,2,3)

- implement basic rubiks cube representation
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
