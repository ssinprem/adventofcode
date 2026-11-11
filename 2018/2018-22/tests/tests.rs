use mode_maze::*;
use utility::test_case_n;

test_case_n!(example,
"depth: 510
target: 10,10",
    part1: (part1::solve, 114),
    part2: (part2::solve, 45)
);
