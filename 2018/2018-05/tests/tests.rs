use alchemical_reduction::*;
use utility::test_case_n;

test_case_n!(example,
"dabAcCaCBAcCcaDA",
    part1: (part1::solve, 10),
    part2: (part2::solve, 4)
);
