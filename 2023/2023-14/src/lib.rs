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
            let row = rows-row;
            let item = map.get(row, col);
            if item == Some(&Cell::Round) {
                let mut found = false;
                let mut j = row + 1;
                loop {
                    let jtem = map.get(j, col);
                    if jtem == Some(&Cell::Empty) {
                        found = true;
                        if j == rows-1 {
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
            let col = cols-col;
            let item = map.get(row, col);
            if item == Some(&Cell::Round) {
                let mut found = false;
                let mut j = col + 1;
                loop {
                    let jtem = map.get(row, j);
                    if jtem == Some(&Cell::Empty) {
                        found = true;
                        if j == cols-1 {
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
        for i in 0..1000 {
            slide_up(&mut map);
            slide_left(&mut map);
            slide_down(&mut map);
            slide_right(&mut map);
            println!("{}", i);
            scores.push(score(&map));
        }
        println!("{:?}", scores);

        let mut i = 0;
        


        score(&map)
    }
}

