use regex::Regex;
use grid::Grid;

pub enum Seq {
    Rect(u32,u32),
    RotCol(u32, u32),
    RotRow(u32, u32),
}

pub fn parse(file: String) -> Vec<Seq> {
    // rect AxB
    // rotate column x=A by B
    // rotate row y=A by B
    let regex_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let regex_col = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    let regex_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();

    file.lines().filter(|line| !line.is_empty())
    .map(|line| {
        if let Some(cap) = regex_rect.captures(line)
        && let Some(x) = cap.get(1)
        && let Ok(x) = x.as_str().parse::<u32>()
        && let Some(y) = cap.get(2)
        && let Ok(y) = y.as_str().parse::<u32>()
        {
            Seq::Rect(x, y)
        } else if let Some(cap) = regex_row.captures(line)
        && let Some(row) = cap.get(1)
        && let Ok(row) = row.as_str().parse::<u32>()
        && let Some(num) = cap.get(2)
        && let Ok(num) = num.as_str().parse::<u32>()
        {
            Seq::RotRow(row, num)
        } else if let Some(cap) = regex_col.captures(line) 
        && let Some(col) = cap.get(1)
        && let Ok(col) = col.as_str().parse::<u32>()
        && let Some(num) = cap.get(2)
        && let Ok(num) = num.as_str().parse::<u32>()
        {
            Seq::RotCol(col, num)
        } else {
            unreachable!();
        }
    }).collect()
}

pub fn _display(maps: &Grid<bool>) -> String {
    let mut output = String::new();
    let rows = maps.rows();
    let cols = maps.cols();

    for row in 0..rows {
        for col in 0..cols {
            let cell = maps.get(row, col).unwrap();
            if *cell {
                output += "#";
            } else {
                output += ".";
            }
        }
        output += "\n";
    }
    output
}

pub mod part1 {
    use super::*;
    use grid::Grid;

    fn process(maps: &mut Grid<bool>, step: Seq) {
        match step {
            Seq::Rect(cols, rows) => {
                for row in 0..rows {
                    for col in 0..cols {
                        let cell = maps.get_mut(row, col).unwrap();
                        *cell = true
                    }
                }
            },
            Seq::RotRow(row, num) => {
                let temp = maps.iter_row(row as usize).copied().collect::<Vec<_>>();
                let mut row = maps.iter_row_mut(row as usize).collect::<Vec<_>>();
                let items = temp.len();
                for i in 0..items {
                    let pos = (items + i - num as usize) % items;
                    let cell = row.get_mut(i).unwrap();
                    **cell = *temp.get(pos).unwrap();
                }
            }
            Seq::RotCol(col, num) => {
                let temp = maps.iter_col(col as usize).copied().collect::<Vec<_>>();
                let mut col = maps.iter_col_mut(col as usize).collect::<Vec<_>>();
                let items = temp.len();
                for i in 0..items {
                    let pos = (items + i - num as usize) % items;
                    let cell = col.get_mut(i).unwrap();
                    **cell = *temp.get(pos).unwrap();
                }
            }
        }
    }

    pub fn solve(file: String, size: (usize, usize)) -> u64 {
        let steps = parse(file);
        let mut maps = Grid::<bool>::new(size.1, size.0);
        for step in steps {
            process(&mut maps, step);
        }
        println!("{}", _display(&maps));
        maps.iter().filter(|cell| **cell).count() as u64
    }
}
