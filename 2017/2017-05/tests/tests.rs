use a_maze_of_twisty_trampolines_all_alike::*;
use utility::test_case_n;

test_case_n!(example,
"0
3
0
1
-3",
    part1: (part1::solve, 5),
    part2: (part2::solve, 10)
);
