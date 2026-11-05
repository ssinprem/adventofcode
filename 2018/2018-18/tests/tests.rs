use settlers_of_the_north_pole::*;
use utility::test_case_n;

test_case_n!(example,
".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.",
    part1: (part1::solve, 1147),
    part2: (part2::solve, 0)
);
