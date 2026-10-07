use i_heard_you_like_registers::*;
use utility::test_case_n;

test_case_n!(example,
"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
    part1: (part1::solve, 1),
    part2: (part2::solve, 10)
);
