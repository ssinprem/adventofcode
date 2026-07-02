use plutonian_pebbles::*;
use utility::test_case_n;

test_case_n!(example,
"125 17",
    part1: (part1::solve, 55312),
    part2: (part2::solve, 0)
);
