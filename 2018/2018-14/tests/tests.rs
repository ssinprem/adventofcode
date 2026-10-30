use chocolate_charts::*;
use utility::test_case_n;

test_case_n!(example1, "9", part1: (part1::solve, "5158916779".to_string(), &[3,7]));
test_case_n!(example2, "5", part1: (part1::solve, "0124515891".to_string(), &[3,7]));
test_case_n!(example3, "18", part1: (part1::solve, "9251071085".to_string(), &[3,7]));
test_case_n!(example4, "2018", part1: (part1::solve, "5941429882".to_string(), &[3,7]));

test_case_n!(example1, "51589", part2: (part2::solve, 9, &[3,7]));
test_case_n!(example2, "01245", part2: (part2::solve, 5, &[3,7]));
test_case_n!(example3, "92510", part2: (part2::solve, 18, &[3,7]));
test_case_n!(example4, "594142", part2: (part2::solve, 2018, &[3,7]));
