use firwall_rules::*;
use utility::test_case_n;

test_case_n!(example,
"5-8
0-2
4-7",
    part1: (part1::solve, Some(3), 9),
    part2: (part2::solve, 0)
);
