use safe_cracking::*;
use utility::test_case_n;

test_case_n!(example,
"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a",
    part1: (part1::solve, 3, "".to_string())
);
