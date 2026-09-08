use infinite_elves_and_infinite_houses::*;
use utility::test_case_n;

test_case_n!(example,
"150",
    part1: (part1::solve, 8),
    part2: (part2::solve, 0)
);
