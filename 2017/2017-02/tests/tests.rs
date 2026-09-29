use corruption_checksum::*;
use utility::test_case_n;

test_case_n!(example,
"5 1 9 5
7 5 3
2 4 6 8",
    part1: (part1::solve, 18),
    part2: (part2::solve, 0)
);
