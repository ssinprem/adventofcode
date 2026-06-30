use disk_fragmenter::*;
use utility::test_case_n;

test_case_n!(example,
"2333133121414131402",
    part1: (part1::solve, 1928),
    part2: (part2::solve, 0)
);
