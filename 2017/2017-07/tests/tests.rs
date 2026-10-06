use recursive_circus::*;
use utility::test_case_n;

test_case_n!(example,
"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
    part1: (part1::solve, "tknk".to_string()),
    part2: (part2::solve, Some(("ugml".to_string(), 60)))
);

test_case_n!(example_modify,
"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (60) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
    part2: (part2::solve, None)
);
