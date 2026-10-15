use particle_swarm::*;
use utility::test_case_n;

test_case_n!(example,
"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>",
    part1: (part1::solve, 0)
);

test_case_n!(example,
"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>",
    part2: (part2::solve, 1)
);
