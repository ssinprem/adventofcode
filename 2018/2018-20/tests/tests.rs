use a_regular_map::*;
use utility::test_case_n;

test_case_n!(example1,
"^WNE$",
    part1: (part1::solve, 3)
);

test_case_n!(example2,
"^ENWWW(NEEE|SSE(EE|N))$",
    part1: (part1::solve, 10)
);
test_case_n!(example3,
"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$",
    part1: (part1::solve, 18)
);

test_case_n!(example4,
"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$",
    part1: (part1::solve, 23)
);

test_case_n!(example5,
"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
    part1: (part1::solve, 31)
);
