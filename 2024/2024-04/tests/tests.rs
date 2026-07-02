use ceres_search::*;
use utility::test_case_n;

test_case_n!(example,
"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
    part1: (part1::solve, 18),
    part2: (part2::solve, 9)
);
