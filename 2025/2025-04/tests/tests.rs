use aoc_2025_04::*;
use utility::test_case_n;

test_case_n!(example,
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    part1: (part1::solve, 13),
    part2: (part2::solve, 43)
);
