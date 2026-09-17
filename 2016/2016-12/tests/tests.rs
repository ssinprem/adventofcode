use radioisotope_thermoelectric_generators::*;
use utility::test_case_n;

test_case_n!(example,
"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.",
    part1: (part1::solve, 11),
    part2: (part2::solve, 0)
);
