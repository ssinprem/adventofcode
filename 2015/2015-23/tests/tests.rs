use opening_the_turing_lock::*;
use utility::test_case_n;

test_case_n!(example,
"inc a
jio a, +2
tpl a
inc a",
    part1: (part1::solve, (2,0)),
    part2: (part2::solve, 0)
);
