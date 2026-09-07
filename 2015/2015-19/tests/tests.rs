use medicine_for_rudolph::*;
use utility::test_case_n;

test_case_n!(example1,
"H => HO
H => OH
O => HH

HOH",
    part1: (part1::solve, 4),
    part2: (part2::solve, 0)
);

test_case_n!(example2,
"H => HO
H => OH
O => HH

HOHOHO",
    part1: (part1::solve, 7),
    part2: (part2::solve, 0)
);
