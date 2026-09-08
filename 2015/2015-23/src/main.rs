use opening_the_turing_lock::*;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))?;
    println!("part1 = {:?}", part1::solve(file.clone(), 0 ,0));
    println!("part2 = {:?}", part1::solve(file.clone(), 1, 0));
    Ok(())
}
