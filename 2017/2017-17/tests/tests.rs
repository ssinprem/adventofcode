use spinlock::*;
use utility::test_case_n;

test_case_n!(example,
"3",
    part1: (part1::solve, 638),
    part2: (part2::solve, 0)
);
