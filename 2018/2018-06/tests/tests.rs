use chronal_coordinates::*;
use utility::test_case_n;

test_case_n!(example,
"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
    part1: (part1::solve, 17),
    part2: (part2::solve, 0)
);
