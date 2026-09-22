use like_a_rogue::*;
use utility::test_case_n;

test_case_n!(example1,
"..^^.",
    part1: (part1::solve, 6, 3)
);

test_case_n!(example2,
".^^.^.^^^^",
    part1: (part1::solve, 38, 10)
);
