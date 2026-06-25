use historian_hysteria::*;
use utility::test_case_n;

test_case_n!(example,
"
3   4
4   3
2   5
1   3
3   9
3   3
",
    part1: (part1::solve, 2 + 1 + 0 + 1 + 2 + 5),
    part2: (part2::solve, 9 + 4 + 0 + 0 + 9 + 9)
);