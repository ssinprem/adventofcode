use christmas_tree_farm::*;
use utility::test_case_n;

test_case_n!(example,
"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
",
    part1: (part1::solve, 2),
    part2: (part2::solve, 0)
);

test_case_n!(first_line,
"
0:
###
##.
#..

1:
##.
.##
..#

2:
.##
##.
###

3:
###
.##
.##

4:
###
#..
###

5:
###
.#.
###

40x42: 30 30 28 31 29 33
",
    part1: (part1::solve, 1),
    part2: (part2::solve, 0)
);

