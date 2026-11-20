use aoc_2019_12::*;
use utility::test_case_n;

test_case_n!(example1,
"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
    part1: (part1::solve, 179, 10),
    part2: (part2::solve, 2772)
);

test_case_n!(example2,
"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
    part1: (part1::solve, 1940, 100),
    part2: (part2::solve, 4686774924)
);
