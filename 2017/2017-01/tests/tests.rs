use inverse_captcha::*;
use utility::test_case_n;

test_case_n!(example1, "1122", part1: (part1::solve, 3));
test_case_n!(example2, "1111", part1: (part1::solve, 4));
test_case_n!(example3, "1234", part1: (part1::solve, 0));
test_case_n!(example4, "91212129", part1: (part1::solve, 9));

test_case_n!(example1, "1212", part2: (part2::solve, 6));
test_case_n!(example2, "1221", part2: (part2::solve, 0));
test_case_n!(example3, "123425", part2: (part2::solve, 4));
test_case_n!(example4, "123123", part2: (part2::solve, 12));
test_case_n!(example5, "12131415", part2: (part2::solve, 4));
