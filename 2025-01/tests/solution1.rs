use secret_entrance::part1::*;
use utility::test_case;

test_case!(basic, solution1, 3,
"L68
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

test_case!(double_large_rotation, solution1, 0,
"R1000
L2000");

test_case!(boundary_extact_zero_landing, solution1, 3,
"L50
R100
L100
R5");

test_case!(passing_thru_0_without_end_onit, solution1, 0,
"L60
R20
R80
");

test_case!(no_hits, solution1, 0,
"R10
L20
R5
L3");