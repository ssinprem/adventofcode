use an_elephant_named_joseph::*;
use utility::test_case_n;

test_case_n!(example,
"5",
    part1: (part1::solve, 3),
    part2: (part2::solve, 2)
);
