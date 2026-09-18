use a_maze_of_twisty_little_cubicles::*;
use utility::test_case_n;

test_case_n!(example,
"10 7,4",
    part1: (part1::solve, 11),
    part2: (part2::solve, 151)
);
