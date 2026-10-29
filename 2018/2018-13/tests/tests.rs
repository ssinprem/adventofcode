use mine_cart_madness::*;
use utility::test_case_n;

test_case_n!(example,
r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ",
    part1: (part1::solve, (7,3))
);
test_case_n!(example,
r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/",
    part2: (part2::solve, (6,4))
);
