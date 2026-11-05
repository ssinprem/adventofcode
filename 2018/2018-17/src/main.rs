use reservoir_research::*;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))?;
    
    let clays = parse(file);
    let miny = clays.iter().min_by_key(|(_x, y)| *y).unwrap().1;
    let maxy = clays.iter().max_by_key(|(_x, y)| *y).unwrap().1;

    let waters = process(&clays);
    _display(&clays, &waters);

    let part1_result = waters
        .iter()
        .filter(|((_x, y), _)| y >= &miny && y <= &maxy)
        .count();

    let part2_result = waters
        .iter()
        .filter(|((_x, y), w)| y >= &miny && y <= &maxy && w == &&Cell::Full)
        .count()

    println!("part1 = {}", part1_result);
    println!("part2 = {}", part2_result);

    Ok(())
}
