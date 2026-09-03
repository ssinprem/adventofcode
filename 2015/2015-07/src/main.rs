use some_assembly_required::*;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>>{
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))?;
    let hm = part1::solve(file.clone());
    println!("part1 a = {:?}", hm.get("a"));
    let hm = part2::solve(file.clone());
    println!("part2 a = {:?}", hm.get("a"));
    Ok(())
}
