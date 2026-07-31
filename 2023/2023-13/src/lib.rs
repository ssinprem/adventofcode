use grid::Grid;

pub fn parse(file: String) -> Vec<Grid<bool>> {
    file.split("\n\n")
        .map(|pattern| {
            let mut maps = Grid::new(0, 0);
            pattern
                .lines()
                .filter(|line| !line.is_empty())
                .for_each(|line| {
                    maps.push_row(line.chars().map(|char| char == '#').collect());
                });
            maps
        })
        .collect()
}

pub fn _display(map: &Grid<bool>, horz: usize, vert: usize) {
    let cols = map.cols();
    let rows = map.rows();
    println!();
    print!("   ");
    for col in 0..cols {
        print!("{}",(col+1)%10);
    }
    println!();
    print!("   ");
    for col in 0..cols {
        if vert != 0 && col + 1 == vert {
            print!(">");
        } else if vert != 0 && col + 1 == vert + 1 {
            print!("<");
        } else {
            print!(" ");
        }
    }
    println!();
    for row in 0..rows {
        print!("{:<2}",row+1);
        if horz != 0 && row + 1 == horz {
            print!("v");
        } else if horz != 0 && row + 1 == horz + 1 {
            print!("^");
        } else {
            print!(" ");
        }
        for col in 0..cols {
            let i = map.get(row, col);
            if i == Some(&true) {
                print!("#");
            } else {
                print!(".");
            }
        }
        if horz != 0 && row + 1 == horz {
            println!("v");
        } else if horz != 0 && row + 1 == horz + 1 {
            println!("^");
        } else {
            println!();
        }
    }
    print!("   ");
    for col in 0..cols {
        if vert != 0 && col + 1 == vert {
            print!(">");
        } else if vert != 0 && col + 1 == vert + 1 {
            print!("<");
        } else {
            print!(" ");
        }
    }
    println!();
}

pub fn is_mirror(map: &Grid<bool>, vert: bool, fix: usize) -> Option<usize> {
    let cols = map.cols();
    let rows = map.rows();
    let max = if vert { cols } else { rows };
    for i in 0..max - 1 {
        let mut j = 0;
        let mut count = 0;
        loop {
            let seta =  if vert { map.iter_col(i - j) } else { map.iter_row(i - j) };
            let setb =  if vert { map.iter_col(i + 1 + j) } else { map.iter_row(i + 1 + j) };
            count += seta.zip(setb).filter(|(a, b)| a != b).count();
            j += 1;
            if i < j || i + 1 + j > max - 1 {
                if count == fix {
                    return Some(i + 1);
                } else {
                    break;
                }
            }
        }
    }
    None
}
pub mod part1 {
    use crate::*;

    pub fn solve(file: String) -> u64 {
        let maps = parse(file);

        maps.iter()
            .map(|map| {
                if let Some(u) = is_mirror(map, true, 0) {
                    // _display(map, 0, u);
                    u
                } else if let Some(u) = is_mirror(map, false, 0) {
                    // _display(map, u, 0);
                    u * 100
                } else {
                    0
                }
            })
            .sum::<usize>() as u64
    }
}

pub mod part2 {
    use crate::*;

    pub fn solve(file: String) -> u64 {
        let maps = parse(file);

        maps.iter()
            .map(|map| {
                if let Some(u) = is_mirror(map, false, 1) {
                    // _display(map, u, 0);
                    u * 100
                } else if let Some(u) = is_mirror(map, true, 1) {
                    // _display(map, 0, u);
                    u
                } else {
                    0
                }
            })
            .sum::<usize>() as u64
    }
}
