use lobby::part2::*;

#[test]
fn example() {

    let file = "987654321111111
811111111111119
234234234234278
818181911112111".to_string();

    assert_eq!(solve(file), 3121910778619);
}

#[test]
fn oneline() {

    let file = "987654321111111".to_string();

    assert_eq!(solve(file), 987654321111);
}

#[test]
fn long_text() {

    let file = "2555245573282137352766682525526364435746545343523394355638332326665366122245646523573255525564158774".to_string();
    assert_eq!(solve(file), 987564158774);
}