use mull_it_over::*;
use utility::test_case_n;

test_case_n!(example,
"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    part1: (part1::solve, 2*4 + 5*5 + 11*8 + 8*5)
);

test_case_n!(example,
"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    part2: (part2::solve, 2*4 + 8*5)
);