use haunted_wasteland::*;
use utility::test_case_n;

test_case_n!(example,
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
    part1: (part1::solve, 2),
    part2: (part2::solve, 0)
);

test_case_n!(example2,
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
    part1: (part1::solve, 6),
    part2: (part2::solve, 0)
);
