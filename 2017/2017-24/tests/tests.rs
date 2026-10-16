use electromagnetic_moat::*;
use utility::test_case_n;

test_case_n!(example,
"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10",
    part1: (part1::solve, 31),
    part2: (part2::solve, 19)
);
