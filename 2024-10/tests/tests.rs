use hoof_it::*;
use utility::test_case_n;

test_case_n!(example,"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    part1: (part1::solve, 36),
    part2: (part2::solve, 81)
);
