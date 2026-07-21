use trebuchet::*;
use utility::test_case_n;

test_case_n!(example,
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    part1: (part1::solve, 142),
    part2: (part2::solve, 0)
);
