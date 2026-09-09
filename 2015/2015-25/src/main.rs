use let_it_snow::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    println!("part1 = {}", part1::solve(2978, 3083));
    Ok(())
}
