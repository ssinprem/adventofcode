use scrambles_letters_and_hash::*;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))?;
    println!("part1 = {}", part1::solve(file.clone(), "abcdefgh".to_string()));
    println!("part2 = {}", part2::solve(file.clone(), "fbgdceah".to_string()));
    println!(" rev  = {}", part1::solve(file.clone(), "dhaegfbc".to_string()));
    Ok(())
}
