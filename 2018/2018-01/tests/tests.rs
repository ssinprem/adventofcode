use chronal_calibration::*;
use utility::test_case_n;

test_case_n!(example,
"+1
-2
+3
+1",
    part1: (part1::solve, 3),
    part2: (part2::solve, 0)
);
