use signals_and_noise::*;
use utility::test_case_n;

test_case_n!(example,
"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar",
    part1: (part1::solve, "easter".to_string()),
    part2: (part2::solve, 0)
);
