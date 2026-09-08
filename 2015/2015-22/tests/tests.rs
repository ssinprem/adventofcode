use wizard_simulator_20xx::*;
use utility::test_case_n;

test_case_n!(example,
"Hit Points: 13
Damage: 8",
    part1: (part1::solve, 226, 10, 250),
    part2: (part2::solve, 0)
);
test_case_n!(example2,
"Hit Points: 14
Damage: 8",
    part1: (part1::solve, 641, 10, 250),
    part2: (part2::solve, 0)
);
