use parabolic_reflector_dish::*;
use utility::test_case_n;

test_case_n!(example,"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    part1: (part1::solve, 136),
    part2: (part2::solve, 0)
);
