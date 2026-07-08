use linen_layout::*;
use utility::test_case_n;

test_case_n!(example,"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    part1: (part1::solve, 6),
    part2: (part2::solve, 0)
);
