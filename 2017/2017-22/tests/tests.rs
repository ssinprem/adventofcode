use sporifica_virus::*;
use utility::test_case_n;

test_case_n!(example,
"..#
#..
...",
    part1: (part1::solve, 5587),
    part2: (part2::solve, 2511944)
);
