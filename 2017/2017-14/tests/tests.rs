use disk_defragmentation::*;
use utility::test_case_n;

test_case_n!(example,
"flqrgnkx",
    part1: (part1::solve, 8108),
    part2: (part2::solve, 0)
);
