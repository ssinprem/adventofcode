use aoc_2019_09::*;
use utility::test_case_n;

test_case_n!(example1,
"109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
    part1: (part1::solve, "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string())
);
test_case_n!(example2,
"1102,34915192,34915192,7,4,7,99,0",
    part1: (part1::solve, "1219070632396864".to_string())
);
test_case_n!(example3,
"104,1125899906842624,99",
    part1: (part1::solve, "1125899906842624".to_string())
);
