use pulse_propagation::*;
use utility::test_case_n;

test_case_n!(example,
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
    part1: (part1::solve, 32000000)
);

test_case_n!(example2,
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
    part1: (part1::solve, 11687500)
);
