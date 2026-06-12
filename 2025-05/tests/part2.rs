use cafetaria::part2::*;

#[test]
fn example() {

    let file = "3-5
10-14
16-20
12-18

1
5
8
11
17
32".to_string();

    assert_eq!(solve(file), 14);
}