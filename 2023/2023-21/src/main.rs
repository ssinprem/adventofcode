use std::error::Error;
use std::fs::read_to_string;
use step_counter::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))?;
    println!("part1 = {}", part1::solve(file.clone(), 64));
    println!("part2 = {}", part2::solve(file.clone(), 26501365));
    Ok(())
}
