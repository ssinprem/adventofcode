use movie_theater::*;
use utility::test_case_n;

test_case_n!(example,
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    part1: (part1::solve, 50),
    part2: (part2::solve, 24)
);
