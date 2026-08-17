use step_counter::*;
use utility::test_case_n;

test_case_n!(example,
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    part1: (part1::solve, 42),
    part2: (part2::solve, 0)
);
