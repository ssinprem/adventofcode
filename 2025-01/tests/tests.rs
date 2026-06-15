use secret_entrance::*;
use utility::test_case_n;

test_case_n!(basic,
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    part1_1: (part1::solution1, 3),
    part1_2: (part1::solution2, 3),
    part1_3: (part1::solution3, 3),
    part1_4: (part1::solution4, 3),
    part2: (part2::solution1, 6)
);

test_case_n!(no_rotation,
"",
    part1: (part1::solution1, 0),
    part2: (part2::solution1, 0)
);

test_case_n!(single_large_rotation,
"R1000",
    part2: (part2::solution1, 10)
);

test_case_n!(double_large_rotation,
"R1000
L2000",
    part1_1: (part1::solution1, 0),
    part1_2: (part1::solution2, 0),
    part1_3: (part1::solution3, 0),
    part1_4: (part1::solution4, 0),
    part2: (part2::solution1, 30)
);

test_case_n!(boundary_extact_zero_landing,
"
L50
R100
L100
R5
",
    part1_1: (part1::solution1, 3),
    part1_2: (part1::solution2, 3),
    part1_3: (part1::solution3, 3),
    part1_4: (part1::solution4, 3),
    part2: (part2::solution1, 3)
);

test_case_n!(passing_thru_0_without_end_onit,
"L60
R20
R80
",
    part1_1: (part1::solution1, 0),
    part1_2: (part1::solution2, 0),
    part1_3: (part1::solution3, 0),
    part1_4: (part1::solution4, 0),
    part2: (part1::solution4, 0)
);

test_case_n!(no_hits,
"R10
L20
R5
L3",
    part1_1: (part1::solution1, 0),
    part1_2: (part1::solution2, 0),
    part1_3: (part1::solution3, 0),
    part1_4: (part1::solution4, 0),
    part2: (part2::solution1, 0)
);

test_case_n!(back_to_start,
"
R10
L20
R5
L3
",
    part1_1: (part1::solution1, 0),
    part1_2: (part1::solution2, 0),
    part1_3: (part1::solution3, 0),
    part1_4: (part1::solution4, 0),
    part2: (part2::solution1, 0)
);

test_case_n!(test_50_0_90,
"
L150
L110
R120
L110
",
    part2: (part2::solution1, 7)
);