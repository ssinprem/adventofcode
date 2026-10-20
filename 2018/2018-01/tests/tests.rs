use chronal_calibration::*;
use utility::test_case_n;

test_case_n!(example1,
"+1
-2
+3
+1",
    part1: (part1::solve, 3)
);

test_case_n!(example1,
"+1
-2
+3
+1",
    part2: (part2::solve, 2)
);

test_case_n!(example2,
"+1
-1",
    part2: (part2::solve, 0)
);
test_case_n!(example3,
"+3
+3
+4
-2
-4",
    part2: (part2::solve, 10)
);
test_case_n!(example4,
"-6
+3
+8
+5
-6",
    part2: (part2::solve, 5)
);
test_case_n!(example5,
"+7
+7
-2
-7
-4",
    part2: (part2::solve, 14)
);
