use aoc_2015_17::*;
use utility::test_case_n;

test_case_n!(example,
"20
15
10
5
5",
    part1: (part1::solve, 4, 25),
    part2: (part2::solve, 3, 25)
);
