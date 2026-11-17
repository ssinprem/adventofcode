use aoc_2015_18::*;
use utility::test_case_n;

test_case_n!(example,
".#.#.#
...##.
#....#
..#...
#.#..#
####..",
    part1: (part1::solve, 4, 4),
    part2: (part2::solve, 17, 5)
);
