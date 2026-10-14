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
    part1: (part1::solve, 4)
);

test_case_n!(example,
"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d",
    part2: (part2::solve, 3)
);
