use high_entropy_passphrases::*;
use utility::test_case_n;

test_case_n!(example,
"aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa",
    part1: (part1::solve, 2)
);

test_case_n!(example,
"abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio",
    part2: (part2::solve, 3)
);
