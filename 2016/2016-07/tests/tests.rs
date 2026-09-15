use internet_protocol_v7::*;
use utility::test_case_n;

test_case_n!(example,
"abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn",
    part1: (part1::solve, 2)
);

test_case_n!(example,
"aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb",
    part2: (part2::solve, 3)
);
