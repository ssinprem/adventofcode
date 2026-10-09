use dueling_generators::*;
use utility::test_case_n;

test_case_n!(example,
"Generator A starts with 65
Generator B starts with 8921",
    part1: (part1::solve, 588),
    part2: (part2::solve, 0)
);
