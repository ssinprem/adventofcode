use subterranean_sustainability::*;
use utility::test_case_n;

test_case_n!(example,
"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",
    part1: (part1::solve, 325),
    part2: (part2::solve, 849999999615)
);
