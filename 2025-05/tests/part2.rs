use cafetaria::part2::*;
use utility::test_case;

test_case!(example, solve, 14,
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

test_case!(no_overlap, solve, 6,
"1-3
10-12

0
1
3
4
10
12
13");

test_case!(complete_swallow_intervals, solve, 10,
"1-10
3-7

2
5
7
11");

test_case!(perfectly_touching_intervals, solve, 16,
"5-15
16-20

4
5
15
16
20
21");