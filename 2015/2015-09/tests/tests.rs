use all_in_a_single_night::*;
use utility::test_case_n;

test_case_n!(example,
"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
    part1: (part1::solve, 605),
    part2: (part2::solve, 982)
);
