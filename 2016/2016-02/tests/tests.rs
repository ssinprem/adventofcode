use aoc_2016_02::*;
use utility::test_case_n;

test_case_n!(example,
"ULL
RRDDD
LURDL
UUUUD",
    part1: (part1::solve, "1985".to_string()),
    part2: (part2::solve, "5DB3".to_string())
);
