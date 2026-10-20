use inventory_management_system::*;
use utility::test_case_n;

test_case_n!(example,
"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab",
    part1: (part1::solve, 4*3),
    part2: (part2::solve, 0)
);
