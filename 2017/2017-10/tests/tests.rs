use knot_hash::*;
use utility::test_case_n;

test_case_n!(example,
"3, 4, 1, 5",
    part1: (part1::solve, 0, 6),
    part2: (part2::solve, 0)
);
