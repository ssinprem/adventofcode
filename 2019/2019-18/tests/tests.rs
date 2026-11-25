use aoc_2019_18::*;
use utility::test_case_n;

test_case_n!(example1,
"#########
#b.A.@.a#
#########",
    part1: (part1::solve, 8),
    part2: (part2::solve, 0)
);

test_case_n!(example2,
"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
    part1: (part1::solve, 86),
    part2: (part2::solve, 0)
);

test_case_n!(example3,
"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
    part1: (part1::solve, 132),
    part2: (part2::solve, 0)
);

test_case_n!(example4,
"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
    part1: (part1::solve, 136),
    part2: (part2::solve, 0)
);

test_case_n!(example5,
"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
    part1: (part1::solve, 81),
    part2: (part2::solve, 0)
);
