use marble_mania::*;
use utility::test_case_n;

test_case_n!(example,
    "9 players; last marble is worth 25 points",
    part1: (part1::solve, 32)
);

test_case_n!(example1,
    "10 players; last marble is worth 1618 points",
    part1: (part1::solve, 8317)
);
test_case_n!(example2,
    "13 players; last marble is worth 7999 points",
    part1: (part1::solve, 146373)
);
test_case_n!(example3,
    "17 players; last marble is worth 1104 points",
    part1: (part1::solve, 2764)
);
test_case_n!(example4,
    "21 players; last marble is worth 6111 points",
    part1: (part1::solve, 54718)
);
test_case_n!(example5,
    "30 players; last marble is worth 5807 points",
    part1: (part1::solve, 37305)
);