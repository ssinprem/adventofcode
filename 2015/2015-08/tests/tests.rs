use matchsticks::*;
use std::fs::read_to_string;

#[test]
fn example(){
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/example.txt"));
    assert_eq!(12, part1::solve(file.unwrap()));
}