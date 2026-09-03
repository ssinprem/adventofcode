use matchsticks::*;
use std::fs::read_to_string;

#[test]
fn example(){
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/example.txt"));
    let string = file.unwrap();
    assert_eq!(12, part1::solve(string.to_string()));
    assert_eq!(19, part2::solve(string.to_string()));
}