use monkey_market::*;
use utility::test_case_n;

test_case_n!(example,"
1
10
100
2024",
    part1: (part1::solve, 37327623),
    part2: (part2::solve, 0)
);

#[test]
fn test_example() {
    assert_eq!(prune(100000000), 16113920);
    let temp = process(123);
    assert_eq!(temp, 15887950);
    let temp = process(temp);
    assert_eq!(temp, 16495136);
    let temp = process(temp);
    assert_eq!(temp, 527345);
    let temp = process(temp);
    assert_eq!(temp, 704524);
    let temp = process(temp);
    assert_eq!(temp, 1553684);
    let temp = process(temp);
    assert_eq!(temp, 12683156);
    let temp = process(temp);
    assert_eq!(temp, 11100544);
    let temp = process(temp);
    assert_eq!(temp, 12249484);
    let temp = process(temp);
    assert_eq!(temp, 7753432);
    let temp = process(temp);
    assert_eq!(temp, 5908254);

    assert_eq!((0..2000).fold(1, |init, _| process(init)), 8685429);
    assert_eq!((0..2000).fold(10, |init, _| process(init)), 4700978);
    assert_eq!((0..2000).fold(100, |init, _| process(init)), 15273692);
    assert_eq!((0..2000).fold(2024, |init, _| process(init)), 8667524);
}
