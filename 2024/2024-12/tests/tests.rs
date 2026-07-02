use garden_groups::*;
use utility::test_case_n;

test_case_n!(first_example,"
AAAA
BBCD
BBCC
EEEC",
    part1: (part1::solve, 4*10 + 4*8 + 4*10 + 1*4 + 3*8),
    part2: (part2::solve, 16 + 16 + 32 + 4 + 12 )
);

test_case_n!(second_example,"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
    part1: (part1::solve, 21*36 + 4 * (1*4)),
    part2: (part2::solve, 436)
);

test_case_n!(eshap_example,"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
    part2: (part2::solve, 236)
);

test_case_n!(larger_example,"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    part1: (part1::solve, 1930),
    part2: (part2::solve, 1206)
);
