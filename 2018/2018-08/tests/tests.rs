use memory_maneuver::*;
use utility::test_case_n;

test_case_n!(example,
"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2",
    part1: (part1::solve, 138),
    part2: (part2::solve, 0)
);
