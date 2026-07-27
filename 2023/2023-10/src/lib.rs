use grid::*;

#[derive(Debug, PartialEq, Clone)]
pub enum D {
    N,
    E,
    W,
    S,
    None,
}

pub fn ops(d: &D) -> D {
    match d {
        D::N => D::S,
        D::E => D::W,
        D::W => D::E,
        D::S => D::N,
        _ => D::None,
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum Cell {
    Start, // S
    Pipe(D, D),
    #[default]
    Ground, // .
}

pub fn parse(file: String) -> Grid<Cell> {
    let mut maps = Grid::new(0, 0);
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let row = line
                .chars()
                .map(|char| match char {
                    'S' => Cell::Start,
                    '|' => Cell::Pipe(D::N, D::S),
                    '-' => Cell::Pipe(D::W, D::E),
                    'J' => Cell::Pipe(D::N, D::W),
                    'L' => Cell::Pipe(D::N, D::E),
                    '7' => Cell::Pipe(D::S, D::W),
                    'F' => Cell::Pipe(D::S, D::E),
                    _ => Cell::Ground,
                })
                .collect();
            maps.push_row(row);
        });
    maps
}

pub fn _display(maps: &Grid<Cell>, curr: &[(isize, isize)]) {
    println!();
    maps.iter_rows().enumerate().for_each(|(y, row)| {
        for (x, cell) in row.enumerate() {
            if curr.contains(&(y as isize, x as isize)) {
                print!(
                    "{}",
                    match *cell {
                        Cell::Start => "$",
                        Cell::Pipe(D::N, D::S) | Cell::Pipe(D::S, D::N) => "║",
                        Cell::Pipe(D::W, D::E) | Cell::Pipe(D::E, D::W) => "═",
                        Cell::Pipe(D::N, D::W) | Cell::Pipe(D::W, D::N) => "╝",
                        Cell::Pipe(D::N, D::E) | Cell::Pipe(D::E, D::N) => "╚",
                        Cell::Pipe(D::S, D::W) | Cell::Pipe(D::W, D::S) => "╗",
                        Cell::Pipe(D::S, D::E) | Cell::Pipe(D::E, D::S) => "╔",
                        Cell::Ground => "•",
                        _ => ".",
                    }
                );
            } else {
                print!(
                    "{}",
                    match *cell {
                        Cell::Start => "S",
                        Cell::Pipe(D::N, D::S) | Cell::Pipe(D::S, D::N) => "│",
                        Cell::Pipe(D::W, D::E) | Cell::Pipe(D::E, D::W) => "─",
                        Cell::Pipe(D::N, D::W) | Cell::Pipe(D::W, D::N) => "┘",
                        Cell::Pipe(D::N, D::E) | Cell::Pipe(D::E, D::N) => "└",
                        Cell::Pipe(D::S, D::W) | Cell::Pipe(D::W, D::S) => "┐",
                        Cell::Pipe(D::S, D::E) | Cell::Pipe(D::E, D::S) => "┌",
                        Cell::Ground => ".",
                        _ => ".",
                    }
                );
            }
        }
        println!();
    });
}

pub fn get_start_type(maps: &Grid<Cell>, start: (isize, isize)) -> Cell {
    let allow: Vec<_> = [D::N, D::E, D::W, D::S]
        .iter()
        .filter(|d| {
            let offset = match d {
                D::N => (-1, 0),
                D::E => (0, 1),
                D::W => (0, -1),
                D::S => (1, 0),
                _ => unreachable!(),
            };
            if let Some(Cell::Pipe(a, b)) = maps.get(start.0 + offset.0, start.1 + offset.1) {
                return *a == ops(d) || *b == ops(d);
            }
            false
        })
        .collect();
    println!("{allow:?}");
    Cell::Pipe(allow[0].clone(), allow[1].clone())
}

pub fn go_next(maps: &Grid<Cell>, mut curr: (isize, isize), mut from: D) -> ((isize, isize), D) {
    if from == D::None
        && let Some(cell) = maps.get(curr.0, curr.1)
        && let Cell::Pipe(a, _b) = cell
    {
        from = a.clone();
    }
    match from {
        D::N => {
            curr = (curr.0 - 1, curr.1);
        }
        D::E => {
            curr = (curr.0, curr.1 + 1);
        }
        D::W => {
            curr = (curr.0, curr.1 - 1);
        }
        D::S => {
            curr = (curr.0 + 1, curr.1);
        }
        _ => unreachable!(),
    };
    let next_cell = maps.get(curr.0, curr.1).unwrap();
    if let Cell::Pipe(a, b) = next_cell {
        if from == ops(a) {
            from = b.clone();
        } else if from == ops(b) {
            from = a.clone();
        } else {
            unreachable!()
        }
    }
    (curr, from)
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let mut maps = parse(file);
        let width = maps.cols();
        let start = maps.iter().position(|cell| *cell == Cell::Start).unwrap();
        let start = ((start / width) as isize, (start % width) as isize);
        let mut curr = start;
        _display(&maps, &[start]);

        let value = get_start_type(&maps, start);
        let start_cell = maps.get_mut(start.0, start.1).unwrap();
        *start_cell = value;

        _display(&maps, &[start]);

        let mut count = 0;
        let mut from = D::None;
        while curr != start || count == 0 {
            (curr, from) = go_next(&maps, curr, from);
            count += 1;
        }
        count / 2
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let mut maps = parse(file);
        let width = maps.cols();
        let height = maps.rows();
        let start = maps.iter().position(|cell| *cell == Cell::Start).unwrap();
        let start = ((start / width) as isize, (start % width) as isize);
        let mut curr = start;
        _display(&maps, &[]);

        let value = get_start_type(&maps, start);
        let start_cell = maps.get_mut(start.0, start.1).unwrap();
        *start_cell = value;

        _display(&maps, &[]);

        let mut path = Vec::<(isize, isize)>::new();
        let mut from = D::None;
        while curr != start || from == D::None {
            (curr, from) = go_next(&maps, curr, from);
            path.push(curr);
        }
        let mut path_map = maps;
        for row in 0..height as isize {
            for col in 0..width as isize {
                if path.iter().all(|(r, c)| (*r, *c) != (row, col)) {
                    let remove = path_map.get_mut(row, col).unwrap();
                    *remove = Cell::Ground
                }
            }
        }
        _display(&path_map, &[]);

        let get_dir = |from: (isize, isize), to: (isize, isize)| -> D {
            match (to.0 - from.0, to.1 - from.1) {
                (-1, 0) => D::N,
                (1, 0) => D::S,
                (0, -1) => D::W,
                (0, 1) => D::E,
                _ => panic!("Invalid connection"),
            }
        };

        // Deduce S pipe type
        let p1 = path[0];
        let p2 = path[path.len() - 2];
        let d1 = get_dir(start, p1);
        let d2 = get_dir(start, p2);
        let s_pipe = Cell::Pipe(d1, d2);

        let mut enclosed_list = Vec::new();
        for row in 0..height as isize {
            let mut crossings = 0;
            for col in 0..width as isize {
                let cell = path_map.get(row, col).unwrap();
                match cell {
                    Cell::Ground => {
                        if crossings % 2 == 1 {
                            enclosed_list.push((row, col));
                        }
                    }
                    Cell::Start => {
                        if let Cell::Pipe(a, b) = &s_pipe
                            && (*a == D::N || *b == D::N)
                        {
                            crossings += 1;
                        }
                    }
                    Cell::Pipe(a, b) => {
                        if *a == D::N || *b == D::N {
                            crossings += 1;
                        }
                    }
                }
            }
        }
        _display(&path_map, enclosed_list.as_slice());
        enclosed_list.len() as u64
    }
}
