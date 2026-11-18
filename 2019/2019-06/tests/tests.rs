use aoc_2019_06::*;
use utility::test_case_n;

test_case_n!(example,
"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
    part1: (part1::solve, 42)
);
test_case_n!(example,
"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN",
    part2: (part2::solve, 4)
);
