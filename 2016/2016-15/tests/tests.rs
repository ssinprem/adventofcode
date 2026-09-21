use timing_is_everything::*;
use utility::test_case_n;

test_case_n!(example,
"Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.",
    part1: (part1::solve, 5),
    part2: (part2::solve, 85)
);
