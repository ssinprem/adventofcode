use aoc_2019_16::*;
use utility::test_case_n;

test_case_n!(example1,
    "80871224585914546619083218645595",
    part1: (part1::solve, 24176176)
);
test_case_n!(example2,
    "19617804207202209144916044189917",
    part1: (part1::solve, 73745418)
);
test_case_n!(example3,
    "69317163492948606335995924319873",
    part1: (part1::solve, 52432133)
);

test_case_n!(example1,
    "03036732577212944063491565474664",
    part2: (part2::solve, 84462026)
);
test_case_n!(example2,
    "02935109699940807407585447034323",
    part2: (part2::solve, 78725270)
);
test_case_n!(example3,
    "03081770884921959731165446850517",
    part2: (part2::solve, 53553731)
);

#[test]
fn test_mutiply() {
    assert_eq!(get_multipy(1, 8), vec![1, 0, -1, 0, 1, 0, -1, 0]);
    assert_eq!(get_multipy(2, 8), vec![0, 1, 1, 0, 0, -1, -1, 0]);
    assert_eq!(get_multipy(3, 8), vec![0, 0, 1, 1, 1, 0, 0, 0]);
    assert_eq!(get_multipy(4, 8), vec![0, 0, 0, 1, 1, 1, 1, 0]);
    assert_eq!(get_multipy(5, 8), vec![0, 0, 0, 0, 1, 1, 1, 1]);
    assert_eq!(get_multipy(6, 8), vec![0, 0, 0, 0, 0, 1, 1, 1]);
    assert_eq!(get_multipy(7, 8), vec![0, 0, 0, 0, 0, 0, 1, 1]);
    assert_eq!(get_multipy(8, 8), vec![0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(
        get_multipy(1, 16),
        vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0]
    );
    assert_eq!(
        get_multipy(2, 16),
        vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1, 0]
    );
    assert_eq!(
        get_multipy(3, 16),
        vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1, 1]
    );
    assert_eq!(
        get_multipy(4, 16),
        vec![0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, -1, -1, -1, -1, 0]
    );
    assert_eq!(
        get_multipy(5, 16),
        vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, -1, -1]
    );
    assert_eq!(
        get_multipy(6, 16),
        vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        get_multipy(7, 16),
        vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0]
    );
    assert_eq!(
        get_multipy(8, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0]
    );
    assert_eq!(
        get_multipy(9, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        get_multipy(10, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        get_multipy(11, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        get_multipy(12, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        get_multipy(13, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1]
    );
    assert_eq!(
        get_multipy(14, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1]
    );
    assert_eq!(
        get_multipy(15, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1]
    );
    assert_eq!(
        get_multipy(16, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    );
    assert_eq!(
        get_multipy(17, 16),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}
