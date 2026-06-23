use std::fs::read_to_string;
use std::error::Error;
use secret_entrance::*;

fn main() -> Result<(), Box<dyn Error>>{
    let file = read_to_string("/work/home/ssinprem/Rust-Dev/adventoofcode/2025-01/input.txt")?;
    println!("1_1 = {}", part1::solution1(file.clone()));
    println!("1_2 = {}", part1::solution2(file.clone()));
    println!("1_3 = {}", part1::solution3(file.clone()));
    println!("1_4 = {}", part1::solution4(file.clone()));
    println!("====================================");
    println!("2_1 = {}", part2::solution1(file.clone()));
    Ok(())
}

