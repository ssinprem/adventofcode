use lobby::part1::*;

#[test]
fn example() {

    let file = "987654321111111
811111111111119
234234234234278
818181911112111".to_string();

    assert_eq!(solve(file), 357);
}