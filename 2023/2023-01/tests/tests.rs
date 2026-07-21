use trebuchet::*;
use utility::test_case_n;

test_case_n!(example,
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    part1: (part1::solve, 142)
);

test_case_n!(example,
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    part2: (part2::solve, 281
    )
);
