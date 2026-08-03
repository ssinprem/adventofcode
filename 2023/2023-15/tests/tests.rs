use lens_library::*;
use utility::test_case_n;

test_case_n!(example,
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
    part1: (part1::solve, 1320),
    part2: (part2::solve, 145)
);
