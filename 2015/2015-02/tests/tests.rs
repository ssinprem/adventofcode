use i_was_told_there_would_be_no_math::*;
use utility::test_case_n;

test_case_n!(example,
"2x3x4",
    part1: (part1::solve, 58)
);

test_case_n!(example_2,
"1x1x10",
    part1: (part1::solve, 43)
);
