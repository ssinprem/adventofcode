use no_matter_how_you_slice_it::*;
use utility::test_case_n;

test_case_n!(example,
"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
    part1: (part1::solve, 4),
    part2: (part2::solve, 0)
);
