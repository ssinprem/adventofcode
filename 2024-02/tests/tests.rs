use rednosed_reports::*;
use utility::test_case_n;

test_case_n!(example,
"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
    part1: (part1::solve, 2),
    part2: (part2::solve, 4)
);