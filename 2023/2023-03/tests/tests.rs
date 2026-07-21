use gear_ratios::*;
use utility::test_case_n;

test_case_n!(example,
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    part1: (part1::solve, 4361),
    part2: (part2::solve, 0)
);
