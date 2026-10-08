use packet_scanners::*;
use utility::test_case_n;

test_case_n!(example,
"0: 3
1: 2
4: 4
6: 4",
    part1: (part1::solve, 24),
    part2: (part2::solve, 0)
);
