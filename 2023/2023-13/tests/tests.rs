use point_of_incidence::*;
use utility::test_case_n;

test_case_n!(example,"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    part1: (part1::solve, 405),
    part2: (part2::solve, 0)
);
