use mirage_maintenance::*;
use utility::test_case_n;

test_case_n!(example,
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    part1: (part1::solve, 114),
    part2: (part2::solve, 0)
);
