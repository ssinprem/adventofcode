use trash_compactor::*;
use utility::test_case_n;

test_case_n!(example,
"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
    part1: (part1::solve, 4277556),
    part2: (part2::solve, 3263827)
);

test_case_n!(single_column,
"
2
3
4
+",
    part1: (part1::solve, 9),
    part2: (part2::solve, 234)
);

test_case_n!(single_row,
"
56 78
* +",
    part1: (part1::solve, 134),
    part2: (part2::solve, 45)
);

test_case_n!(interleaved_internal_spaces,
"
    12
3 
    45
+",
    part1: (part1::solve, 60),
    part2: (part2::solve, 42)
);
