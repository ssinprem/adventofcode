use aoc_2019_08::*;
use utility::test_case_n;

test_case_n!(example,
"123456789012",
    part1: (part1::solve, 1, (3,2)),
    part2: (part2::solve, 0)
);
