use aoc_2019_08::*;
use utility::test_case_n;

test_case_n!(example,
    "123456789012",
    part1: (part1::solve, 1, (3,2))
);

test_case_n!(example,
    "0222112222120000",
    part2: (part2::solve, " #\n# ".to_string(), (2,2))
);