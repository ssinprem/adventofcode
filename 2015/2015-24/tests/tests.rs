use it_hangs_in_the_balance::*;
use utility::test_case_n;

test_case_n!(example,
"1
2
3
4
5
7
8
9
10
11",
    part1: (part1::solve, 99),
    part2: (part2::solve, 44)
);
