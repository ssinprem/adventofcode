use never_tell_me_the_odds::*;
use utility::test_case_n;

test_case_n!(example,
"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
    part1: (part1::solve, 2, 7, 27),
    part2: (part2::solve, 47)
);
