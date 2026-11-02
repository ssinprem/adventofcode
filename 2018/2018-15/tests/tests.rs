use beverage_bandits::*;
use utility::test_case_n;

test_case_n!(example1,
"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
    part1: (part1::solve, 47  * 590),
    part2: (part2::solve, 0)
);

test_case_n!(exampl2,
"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
    part1: (part1::solve, 37 * 982),
    part2: (part2::solve, 0)
);

test_case_n!(exampl3,
"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
    part1: (part1::solve, 46 * 859),
    part2: (part2::solve, 0)
);

test_case_n!(exampl4,
"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
    part1: (part1::solve, 35 * 793),
    part2: (part2::solve, 0)
);

test_case_n!(exampl5,
"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
    part1: (part1::solve, 54 * 536),
    part2: (part2::solve, 0)
);

test_case_n!(exampl6,
"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
    part1: (part1::solve, 20 * 937),
    part2: (part2::solve, 0)
);


