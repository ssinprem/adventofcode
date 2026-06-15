use lobby::*;
use utility::test_case_n;

test_case_n!(example,
"987654321111111
811111111111119
234234234234278
818181911112111",
    part1: (part1::solve, 357),
    part2: (part2::solve, 3121910778619)
);

test_case_n!(oneline,
"987654321111111",
    part1: (part1::solve, 98),
    part2: (part2::solve, 987654321111)
);

test_case_n!(long_text,
"2555245573282137352766682525526364435746545343523394355638332326665366122245646523573255525564158774",
    part1: (part1::solve, 98),
    part2: (part2::solve, 987564158774)
);