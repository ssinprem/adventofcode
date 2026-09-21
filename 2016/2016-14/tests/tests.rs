use one_time_pad::*;
use utility::test_case_n;

test_case_n!(example,
"abc",
    part1: (part1::solve, 22728),
    part2: (part2::solve, 0)
);
