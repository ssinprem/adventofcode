use bridge_repair::*;
use utility::test_case_n;

test_case_n!(example,
"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
    part1: (part1::solve, 190 + 3267 + 292),
    part2: (part2::solve, 11387)
);
