use perfectly_spherical_houses_in_a_vacuum::*;
use utility::test_case_n;

test_case_n!(example1,
">",
    part1: (part1::solve, 2)
);
test_case_n!(example1,
"^v",
    part2: (part2::solve, 3)
);
test_case_n!(example2,
"^>v<",
    part1: (part1::solve, 4),
    part2: (part2::solve, 3)
);
test_case_n!(example3,
"^v^v^v^v^v",
    part1: (part1::solve, 2),
    part2: (part2::solve, 11)
);
