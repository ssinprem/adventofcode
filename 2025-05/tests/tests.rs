use cafetaria::*;
use utility::test_cases;

test_cases!(example, 
    part1::solve, 3,
    part2::solve, 14,
"3-5
10-14
16-20
12-18

1
5
8
11
17
32");

test_cases!(no_overlap, 
    part1::solve, 4,
    part2::solve, 6,
"1-3
10-12

0
1
3
4
10
12
13");

test_cases!(complete_swallow_intervals,
    part1::solve, 3,
    part2::solve, 10,
"1-10
3-7

2
5
7
11");

test_cases!(perfectly_touching_intervals,
    part1::solve, 4,
    part2::solve, 16,
"5-15
16-20

4
5
15
16
20
21");