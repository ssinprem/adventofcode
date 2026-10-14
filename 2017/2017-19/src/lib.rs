use grid::Grid;

pub fn parse(file: String) -> Grid<char> {
    let mut map = Grid::new(0, 0);
    file.lines().for_each(|line| {
        map.push_row(line.chars().collect());
    });
    map
}

pub fn process(map: Grid<char>) -> String {
    let mut output = String::new();
    let cols = map.cols();
    let curr = (0..cols)
        .find_map(|col| {
            if let Some(char) = map.get(0, col)
                && char == &'|'
            {
                Some((0_i32, col as i32))
            } else {
                None
            }
        })
        .unwrap();

    let mut stack = vec![(curr, (1, 0))];
    while let Some(((row, col), (dy, dx))) = stack.pop() {
        if let Some(curr) = map.get(row, col) {
            // println!("'{curr}' {row:2},{col:2} ({dy:2},{dx:2})");
            match *curr {
                '|' | '-' => {
                    output += format!("{curr}").as_str();
                    stack.push(((row + dy, col + dx), (dy, dx)));
                }
                '+' => {
                    output += format!("{curr}").as_str();
                    for (ny, nx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        if ny != -dy && nx != -dx {
                            stack.push(((row + ny, col + nx), (ny, nx)));
                        }
                    }
                }
                ' ' => {
                    // do nothing
                }
                c => {
                    output += format!("{c}").as_str();
                    stack.push(((row + dy, col + dx), (dy, dx)));
                }
            }
        }
    }
    output
}
pub mod part1 {
    use crate::{parse, process};

    pub fn solve(file: String) -> String {
        let map = parse(file);
        let output = process(map);
        output
            .chars()
            .filter(|char| !matches!(*char, '|' | '-' | '+'))
            .collect::<String>()
    }
}

pub mod part2 {
    use crate::{parse, process};

    pub fn solve(file: String) -> usize {
        let map = parse(file);
        let output = process(map);
        output.len()
    }
}
