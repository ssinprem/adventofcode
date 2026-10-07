use hex_ed::*;
use utility::test_case_n;

test_case_n!(example1,
"ne,ne,ne",
    part1: (part1::solve, 3),
    part2: (part2::solve, 3)
);

test_case_n!(example2,
"ne,ne,sw,sw",
    part1: (part1::solve, 0),
    part2: (part2::solve, 2)
);
test_case_n!(example3,
"ne,ne,s,s",
    part1: (part1::solve, 2),
    part2: (part2::solve, 2)
);

test_case_n!(example4,
"se,sw,se,sw,sw",
    part1: (part1::solve, 3),
    part2: (part2::solve, 3)
);
