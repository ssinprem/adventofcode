use no_time_for_a_taxicab::*;
use utility::test_case_n;

test_case_n!(example1, "R2, L3", part1: (part1::solve, 5));
test_case_n!(example2, "R2, R2, R2", part1: (part1::solve, 2));
test_case_n!(example3, "R5, L5, R5, R3", part1: (part1::solve, 12));
