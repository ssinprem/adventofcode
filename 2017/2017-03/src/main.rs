use spiral_memory::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("part1 = {}", part1::solve(277678));
    println!("part2 = {}", part2::solve(277678));
    Ok(())
}
