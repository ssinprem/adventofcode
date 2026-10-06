use memory_reallocation::*;
use utility::test_case_n;

test_case_n!(example,
"0 2 7 0",
    part1: (part1::solve, 5),
    part2: (part2::solve, 4)
);
