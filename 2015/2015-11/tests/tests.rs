use corparate_policy::*;
use utility::test_case_n;

test_case_n!(example1,
"hijklmmn",
    part1: (part1::solve, "hjaaabcc"),
    part2: (part2::solve, 0)
);
test_case_n!(example2,
"abbceffg",
    part1: (part1::solve, "abbcefgg"),
    part2: (part2::solve, 0)
);
test_case_n!(example3,
"abbcegjk",
    part1: (part1::solve, "abbcffgh"),
    part2: (part2::solve, 0)
);
test_case_n!(example4,
"abcdefgh",
    part1: (part1::solve, "abcdffaa"),
    part2: (part2::solve, 0)
);
test_case_n!(example5,
"ghijklmn",
    part1: (part1::solve, "ghjaabcc"),
    part2: (part2::solve, 0)
);
