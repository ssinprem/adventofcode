use trash_compactor::*;
use utility::test_cases;

test_cases!(example,
    part1::solve, 4277556,
    part2::solve, 3263827,
"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ");

test_cases!(single_column,
    part1::solve, 9,
    part2::solve, 234,
"
2
3
4
+");

test_cases!(single_row,
    part1::solve, 134,
    part2::solve, 45,
"
56 78
* +");

test_cases!(interleaved_internal_spaces,
    part1::solve, 60,
    part2::solve, 42,
"
 12
3 
 45
+");

