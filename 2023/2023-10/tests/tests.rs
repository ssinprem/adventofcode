use pipe_maze::*;
use utility::test_case_n;

test_case_n!(example,"
.....
.S-7.
.|.|.
.L-J.
.....",
    part1: (part1::solve, 4)
);

test_case_n!(example2,"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
    part1: (part1::solve, 8)
);
