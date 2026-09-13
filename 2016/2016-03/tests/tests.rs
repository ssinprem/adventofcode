use squares_with_three_sides::*;
use utility::test_case_n;

test_case_n!(example,
"5 10 25",
    part1: (part1::solve, 0)
);

test_case_n!(example,
"101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603",
    part2: (part2::solve, 6)
);
