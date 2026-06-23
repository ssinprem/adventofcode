use std::error::Error;
use std::fs::read_to_string;
use reactor::*;

fn main() -> Result<(), Box<dyn Error>>{
    let file = read_to_string("/work/home/ssinprem/Rust-Dev/adventoofcode/2025-11/input.txt")?;
    println!("part1 = {}", part1::solve(file.clone()));
    println!("part2 = {}", part2::solve(file.clone()));
    Ok(())
}