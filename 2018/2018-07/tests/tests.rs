use the_sum_of_its_parts::*;
use utility::test_case_n;

test_case_n!(example,
"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
    part1: (part1::solve, "CABDFE".to_string()),
    part2: (part2::solve, 15, 2, 0)
);
