use the_tyranny_of_the_rocket_equation::*;
use utility::test_case_n;

#[test]
fn test() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}

test_case_n!(example,
"",
    part1: (part1::solve, 0),
    part2: (part2::solve, 0)
);
