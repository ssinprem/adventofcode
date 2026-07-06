use reindeer_maze::*;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))?;
    println!("part1 = {}", part1::solve(file.clone()));
    println!("part2 = {}", part2::solve(file.clone()));
    Ok(())
}
