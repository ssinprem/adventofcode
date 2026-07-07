use chronospatial_computer::*;
use utility::test_case_n;

test_case_n!(example,"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    part1: (part1::solve, "4,6,3,5,6,3,5,2,1,0".to_string()),
    part2: (part2::solve, 0)
);

test_case_n!(a10, "
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4",
    part1: (part1::solve, "0,1,2")
);

test_case_n!(a2024, "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    part1: (part1::solve, "4,2,5,6,7,7,7,7,3,1,0")
);

test_case_n!(part2, "
Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
    part1: (part1::solve, "0,3,5,4,3,0"),
    part2: (part2::solve, 117440)
);

test_case_n!(bxc, "
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0",
    part1: (part1::solve, "")
);

test_case_n!(bxl_literal_under_4, "
Register A: 0
Register B: 5
Register C: 0

Program: 1,3,5,5",
    part1: (part1::solve, "6")
);

test_case_n!(bst_combo, "
Register A: 0
Register B: 0
Register C: 15

Program: 2,6,5,5",
    part1: (part1::solve, "7")
);

test_case_n!(adv_combo, "
Register A: 32
Register B: 3
Register C: 0

Program: 0,5,5,4",
    part1: (part1::solve, "4")
);

test_case_n!(bxc_only, "
Register A: 0
Register B: 12
Register C: 15

Program: 4,0,5,5",
    part1: (part1::solve, "3")
);

test_case_n!(loop_octal, "
Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
    part1: (part1::solve, "0,3,5,4,3,0")
);

test_case_n!(full_input, "
Register A: 47006051
Register B: 0
Register C: 0

Program: 2,4, 1,3, 7,5, 1,5, 0,3, 4,3, 5,5, 3,0",
    part1: (part1::solve, "6,2,7,2,3,1,6,0,5")
);
