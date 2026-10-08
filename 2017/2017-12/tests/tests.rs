use digital_plumber::*;
use utility::test_case_n;

test_case_n!(example,
"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5",
    part1: (part1::solve, 6),
    part2: (part2::solve, 2)
);
