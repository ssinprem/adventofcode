use science_for_hungry_people::*;
use utility::test_case_n;

test_case_n!(example,
"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
    part1: (part1::solve, 62842880),
    part2: (part2::solve, 57600000)
);
