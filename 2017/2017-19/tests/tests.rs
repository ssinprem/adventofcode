use a_series_of_tubes::*;
use utility::test_case_n;

test_case_n!(example,
"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                ",
    part1: (part1::solve, "ABCDEF".to_string()),
    part2: (part2::solve, 0)
);
