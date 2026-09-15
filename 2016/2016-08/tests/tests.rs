use two_factor_authentication::*;
use utility::test_case_n;

test_case_n!(example,
"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1
",
    part1: (part1::solve, 6
        , (7, 3)),
    part2: (part2::solve, 0)
);
