use experimental_emergency_teleportation::*;
use utility::test_case_n;

test_case_n!(example,
"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1",
    part1: (part1::solve, 7),
    part2: (part2::solve, 0)
);
