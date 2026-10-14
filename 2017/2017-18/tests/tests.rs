use duet::*;
use utility::test_case_n;

test_case_n!(example,
"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2",
    part1: (part1::solve, 4),
    part2: (part2::solve, 0)
);
