use explosives_in_cyberspace::*;
use utility::test_case_n;

test_case_n!(example1, "ADVENT", part1: (part1::solve, 6));
test_case_n!(example2, "A(1x5)BC", part1: (part1::solve, 7));
test_case_n!(example3, "(3x3)XYZ", part1: (part1::solve, 9));
test_case_n!(example4, "A(2x2)BCD(2x2)EFG", part1: (part1::solve, 11));
test_case_n!(example5, "(6x1)(1x3)A", part1: (part1::solve, 6));
test_case_n!(example6, "X(8x2)(3x3)ABCY", part1: (part1::solve, 18));
