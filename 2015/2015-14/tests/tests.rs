use reindeer_olympics::*;
use utility::test_case_n;

test_case_n!(example,
"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
    part1: (part1::solve, 1120, 1000),
    part2: (part2::solve, 0)
);
