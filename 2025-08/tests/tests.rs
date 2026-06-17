use playground::*;
use utility::test_case_n;

test_case_n!(example,
"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
",
    part1: (part1::solve, 40, 10_u64),
    part2: (part2::solve, 0, 10_u64)
);

test_case_n!(basic_connections,
"0,0,0
1,0,0
2,0,0
10,10,10
",
    part1: (part1::solve, 3, 3), // (Size 3) * (Size 1) * (Size 1) = 3 (Adjust logic based on puzzle requirements)
    part2: (part2::solve, 0, 3)
);

// Test case 2: Redundant connections.
// If A-B and B-C are connected, A-C should be ignored.
test_case_n!(redundant_paths,
"0,0,0
1,0,0
2,0,0
",
    part1: (part1::solve, 3, 3),
    part2: (part2::solve, 0, 10)
);

// Test case 3: Ties in distance.
// Ensures stable sorting or handling of equal distances.
test_case_n!(distance_ties,
"0,0,0
0,0,1
10,0,0
10,0,1
",
    part1: (part1::solve, 4, 10),
    part2: (part2::solve, 0, 10)
);

// Test case 4: Disjoint clusters.
// Groups that never bridge to other groups.
test_case_n!(disjoint_groups,
"0,0,0
0,0,1
100,100,100
100,100,101
",
    part1: (part1::solve, 4, 2), // 2 * 2 * 1
    part2: (part2::solve, 0, 10)
);

// Test case 5: Star topology.
// One central point connecting to several others.
test_case_n!(star_topology,
"0,0,0
1,0,0
0,1,0
0,0,1
",
    part1: (part1::solve, 4, 4),
    part2: (part2::solve, 0, 10)
);

test_case_n!(ten_connection_test,
"0,0,0
1,0,0
0,1,0
0,0,1
10,10,10
11,10,10
10,11,10
10,10,11
20,20,20
21,20,20
20,21,20
20,20,21
",
    // With 12 boxes, there are 66 possible pairs. 
    // After exactly 10 connections, calculate the 3 largest circuits.
    part1: (part1::solve, 64, 10),
    part2: (part2::solve, 0, 10)
);