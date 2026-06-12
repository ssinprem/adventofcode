use secret_entrance::part2::*;
use utility::test_case;

test_case!(basic, solution1, 6,
"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
");

test_case!(at_crossing_zero_once, solution1, 4,
"
R60
L20
R20
L20
");

test_case!(over_round, solution1, 6,
"
R100
L100
R200
L200
");

test_case!(no_rotation, solution1, 0, "");

test_case!(single_large_rotation, solution1, 10, "R1000");

test_case!(double_large_rotation, solution1, 30,
"
R1000
L2000");

test_case!(boundary_extact_zero_landing, solution1, 3,
"
L50
R100
L100
R5
");

test_case!(passing_thru_0_without_end_onit, solution1, 2,
"
L60
R20
R80
");

test_case!(no_hits, solution1, 0,
"
R10
L20
R5
L3
");

test_case!(back_to_start, solution1, 0,
"
R10
L20
R10
");

test_case!(test_50_0_90, solution1, 7,
"
L150
L110
R120
L110
");
