use utility::test_case_n;
use aoc_2023_06::*;

test_case_n!(example,
"Time:      7  15   30
Distance:  9  40  200",
    part1: (part1::solve, 288),
    part2: (part2::solve, 71503)
);
