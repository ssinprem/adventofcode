use opening_the_turing_lock::*;
use utility::test_case_n;

test_case_n!(example,
"inc a
jio a, +2
tpl a
inc a",
    part1: (part1::solve, (2,0), 0, 0),
    part2: (part1::solve, (7,0), 1, 0)
);
