use the_ideal_stocking_stuffer::*;
use utility::test_case_n;

test_case_n!(example1,
"abcdef",
    part1: (part1::solve, 609043),
    part2: (part2::solve, 6742839)
);

test_case_n!(example2,
"pqrstuv",
    part1: (part1::solve, 1048970),
    part2: (part2::solve, 5714438)
);
