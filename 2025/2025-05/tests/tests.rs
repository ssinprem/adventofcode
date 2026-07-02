use cafetaria::*;
use utility::test_case_n;

test_case_n!(example,
"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    part1: (part1::solve, 3),
    part2: (part2::solve, 14)
);

test_case_n!(no_overlap,
"1-3
10-12

0
1
3
4
10
12
13",
    part1: (part1::solve, 4),
    part2: (part2::solve, 6)
);

test_case_n!(complete_swallow_intervals,
"1-10
3-7

2
5
7
11",
    part1: (part1::solve, 3),
    part2: (part2::solve, 10)
);

test_case_n!(perfectly_touching_intervals,
"5-15
16-20

4
5
15
16
20
21",
    part1: (part1::solve, 4),
    part2: (part2::solve, 16)
);
