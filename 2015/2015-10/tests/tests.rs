use elves_look_elves_say::*;
use utility::test_case_n;

test_case_n!(example,
"1",
    part1: (part1::solve, 82350, 40),
    part2: (part2::solve, 0)
);
