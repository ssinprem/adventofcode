use aoc_2016_05::*;
use utility::test_case_n;

test_case_n!(example,
"abc",
    part1: (part1::solve, "18f47a30".to_string()),
    part2: (part2::solve, "05ace8e3".to_string())
);
