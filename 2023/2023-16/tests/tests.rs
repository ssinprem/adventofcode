use the_floor_will_be_lava::*;
use utility::test_case_n;

test_case_n!(example,
".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
    part1: (part1::solve, 46),
    part2: (part2::solve, 0)
);
