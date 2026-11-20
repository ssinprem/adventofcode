use std::collections::HashMap;

pub fn _display(pad: &HashMap<(usize, usize), usize>) {
    let min_x = pad.keys().map(|(x, _y)| *x).min().unwrap();
    let max_x = pad.keys().map(|(x, _y)| *x).max().unwrap();
    let min_y = pad.keys().map(|(_x, y)| *y).min().unwrap();
    let max_y = pad.keys().map(|(_x, y)| *y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(v) = pad.get(&(x, y)) {
                print!("{v}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::_display;

    pub fn solve(file: String) -> usize {
        let output = aoc_2019_09::part1::solve(file);
        let mut pad = HashMap::new();

        let output = output
            .split(",")
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        for (x, y, v) in output.chunks(3).map(|chk| (chk[0], chk[1], chk[2])) {
            pad.insert((x, y), v);
        }
        _display(&pad);
        pad.values().filter(|v| v == &&2).count()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
