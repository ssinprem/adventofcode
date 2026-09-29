use air_duct_spelunking::*;
use utility::test_case_n;

test_case_n!(example,
"###########
#0.1.....2#
#.#######.#
#4.......3#
###########",
    part1: (part1::solve, 14),
    part2: (part2::solve, 20)
);
