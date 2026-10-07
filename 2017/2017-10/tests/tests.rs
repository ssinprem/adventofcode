use knot_hash::*;
use utility::test_case_n;

test_case_n!(example,
"3, 4, 1, 5",
    part1: (part1::solve, 0, 6)
);

test_case_n!(example,
"",
    part2: (part2::solve, "a2582a3a0e66e6e86e3812dcb672a272".to_string())
);
test_case_n!(example2,
"AoC 2017",
    part2: (part2::solve, "33efeb34ea91902bb2f59c9920caa6cd".to_string())
);
test_case_n!(example3,
"1,2,3",
    part2: (part2::solve, "3efbe78a8d82f29979031a4aa0b16a9d".to_string())
);
test_case_n!(example4,
"1,2,4",
    part2: (part2::solve, "63960835bcdc130f0b66d7ff4f6a5a8e".to_string())
);
