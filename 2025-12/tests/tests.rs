use christmas_tree_farm::*;
use utility::*;

const EXAMPLE_PATTERN : &str =
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
";

test_case_n!(example_good,
(EXAMPLE_PATTERN.to_string() + "

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
").as_str(),
    part1: (part1::solve, 2)
);

test_case_n_ignore!(example_bad,
(EXAMPLE_PATTERN.to_string() + "

12x5: 1 0 1 0 3 2
").as_str(),
    part1: (part1::solve, 0)
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
    part1: (part1::solve, 1)
);

