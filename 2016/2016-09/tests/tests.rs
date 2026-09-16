use explosives_in_cyberspace::*;
use utility::test_case_n;

test_case_n!(example1, "ADVENT", part1: (part1::solve, 6));
test_case_n!(example2, "A(1x5)BC", part1: (part1::solve, 7));
test_case_n!(example3, "(3x3)XYZ", part1: (part1::solve, 9));
test_case_n!(example4, "A(2x2)BCD(2x2)EFG", part1: (part1::solve, 11));
test_case_n!(example5, "(6x1)(1x3)A", part1: (part1::solve, 6));
test_case_n!(example6, "X(8x2)(3x3)ABCY", part1: (part1::solve, 18));


test_case_n!(example1, "(3x3)XYZ", part2: (part2::solve, 9));
test_case_n!(example2, "X(8x2)(3x3)ABCY", part2: (part2::solve, 20));
test_case_n!(example3, "(27x12)(20x12)(13x14)(7x10)(1x12)A", part2: (part2::solve, 241920));
test_case_n!(example4, "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", part2: (part2::solve, 445));
