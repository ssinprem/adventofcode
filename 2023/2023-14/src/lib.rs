use grid::*;

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub enum Cell {
    #[default]
    Empty,
    Cube,
    Round,
}

pub fn parse(file: String) -> Grid<Cell> {
    let mut map = Grid::new(0, 0);
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            map.push_row(
                line.chars()
                    .map(|char| match char {
                        'O' => Cell::Round,
                        '#' => Cell::Cube,
                        _ => Cell::Empty,
                    })
                    .collect(),
            );
        });
    map
}

pub fn _display(map: &Grid<Cell>) -> String {
    let cols = map.cols();
    let rows = map.rows();
    let mut string = String::new();
    for row in 0..rows {
        for col in 0..cols {
            let item = map.get(row, col);
            match item {
                Some(&Cell::Cube) => string += "#",
                Some(&Cell::Round) => string += "O",
                _ => string += ".",
            }
        }
        string += "\n"
    }
    string
}

pub fn slide_up(map: &mut Grid<Cell>) {
    let rows = map.rows();
    let cols = map.cols();

    for col in 0..cols {
        for row in 1..rows {
            let item = map.get(row, col);
            if item == Some(&Cell::Round) {
                let mut found = false;
                let mut j = row - 1;
                loop {
                    let jtem = map.get(j, col);
                    if jtem == Some(&Cell::Empty) {
                        found = true;
                        if j == 0 {
                            break;
                        }
                        j -= 1;
                    } else {
                        j += 1;
                        break;
                    }
                }
                if found {
                    map.swap((row, col), (j, col));
                }
            }
        }
    }
}

pub fn slide_down(map: &mut Grid<Cell>) {
    let rows = map.rows();
    let cols = map.cols();

    for col in 0..cols {
        for row in 0..=rows {
            let row = rows - row;
            let item = map.get(row, col);
            if item == Some(&Cell::Round) {
                let mut found = false;
                let mut j = row + 1;
                loop {
                    let jtem = map.get(j, col);
                    if jtem == Some(&Cell::Empty) {
                        found = true;
                        if j == rows - 1 {
                            break;
                        }
                        j += 1;
                    } else {
                        j -= 1;
                        break;
                    }
                }
                if found {
                    map.swap((row, col), (j, col));
                }
            }
        }
    }
}

pub fn slide_left(map: &mut Grid<Cell>) {
    let rows = map.rows();
    let cols = map.cols();

    for row in 0..rows {
        for col in 1..cols {
            let item = map.get(row, col);
            if item == Some(&Cell::Round) {
                let mut found = false;
                let mut j = col - 1;
                loop {
                    let jtem = map.get(row, j);
                    if jtem == Some(&Cell::Empty) {
                        found = true;
                        if j == 0 {
                            break;
                        }
                        j -= 1;
                    } else {
                        j += 1;
                        break;
                    }
                }
                if found {
                    map.swap((row, col), (row, j));
                }
            }
        }
    }
}

pub fn slide_right(map: &mut Grid<Cell>) {
    let rows = map.rows();
    let cols = map.cols();

    for row in 0..rows {
        for col in 0..=cols {
            let col = cols - col;
            let item = map.get(row, col);
            if item == Some(&Cell::Round) {
                let mut found = false;
                let mut j = col + 1;
                loop {
                    let jtem = map.get(row, j);
                    if jtem == Some(&Cell::Empty) {
                        found = true;
                        if j == cols - 1 {
                            break;
                        }
                        j += 1;
                    } else {
                        j -= 1;
                        break;
                    }
                }
                if found {
                    map.swap((row, col), (row, j));
                }
            }
        }
    }
}

pub fn score(map: &Grid<Cell>) -> u64 {
    let rows = map.rows();
    map.iter_rows()
        .enumerate()
        .map(|(row, items)| items.filter(|cell| cell == &&Cell::Round).count() * (rows - row))
        .sum::<usize>() as u64
}
pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let mut map = parse(file);
        println!("{}", _display(&map));
        slide_up(&mut map);
        println!("{}", _display(&map));
        let rows = map.rows();
        map.iter_rows()
            .enumerate()
            .map(|(row, items)| items.filter(|cell| cell == &&Cell::Round).count() * (rows - row))
            .sum::<usize>() as u64
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let mut map = parse(file);
        let mut scores = Vec::new();
        // A few hundred iterations should be enough to find a cycle.
        for _ in 0..300 {
            slide_up(&mut map);
            slide_left(&mut map);
            slide_down(&mut map);
            slide_right(&mut map);
            scores.push(score(&map));
        }

        let (cycle_start, cycle_len) = {
            let mut cycle_start = 0;
            let mut cycle_len = 0;
            // Find a repeating pattern. To be certain, we look for a sequence that repeats 3 times.
            'outer: for len in 2..scores.len() / 3 {
                let window_size = len * 3;
                if scores.len() < window_size {
                    continue;
                }

                for i in (0..=scores.len() - window_size).rev() {
                    let s1 = &scores[i..i + len];
                    let s2 = &scores[i + len..i + 2 * len];
                    let s3 = &scores[i + 2 * len..i + 3 * len];

                    if s1 == s2 && s2 == s3 {
                        // Pattern found. Now, find the actual start of the cycle.
                        let mut start = i;
                        while start > 0 && scores[start - 1] == scores[start - 1 + len] {
                            start -= 1;
                        }
                        cycle_start = start;
                        cycle_len = len;
                        break 'outer;
                    }
                }
            }
            (cycle_start, cycle_len)
        };

        if cycle_len > 0 {
            let target_cycles = 1_000_000_000;
            // The score for the Nth cycle is at index (N-1)
            let target_idx_0_based = target_cycles - 1;
            let offset_in_cycle = (target_idx_0_based - cycle_start) % cycle_len;
            return scores[cycle_start + offset_in_cycle];
        }

        // Fallback in case no cycle is found.
        *scores.last().unwrap_or(&0)
    }
}
