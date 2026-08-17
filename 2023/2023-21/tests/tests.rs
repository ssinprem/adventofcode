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
    part1_6: (part1::solve, 16, 6),
    part1_64: (part1::solve, 42, 64),
    part2_6: (part2::solve, 16, 6),
    part2_10: (part2::solve, 50, 10),
    part2_50: (part2::solve, 1594, 50),
    part2_100: (part2::solve, 6536, 100),
    part2_500: (part2::solve, 167004, 500)
    // part2_1000: (part2::solve, 668697, 1000),
    // part2_5000: (part2::solve, 16733044, 5000)
);
