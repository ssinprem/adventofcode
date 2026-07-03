use grid::*;

#[derive(Default, Debug, PartialEq)]
pub enum Cell {
    #[default]
    Unknown,
    Space,
    Bubble,
    Block,
    Puffy,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse(file: String) -> (Grid<Cell>, Vec<Direction>) {
    let paragraph = file.split_once("\n\n").expect("cannot split paragraph");

    let cols = paragraph
        .0
        .lines()
        .find(|line| !line.is_empty())
        .expect("cannot get first line")
        .len();
    let mut grid = Grid::new(0, cols);
    paragraph
        .0
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            grid.push_row(
                line.chars()
                    .map(|c| match c {
                        '@' => Cell::Puffy,
                        '.' => Cell::Space,
                        '#' => Cell::Block,
                        'O' => Cell::Bubble,
                        _ => Cell::Unknown,
                    })
                    .collect(),
            );
        });

    let steps = paragraph
        .1
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    (grid, steps)
}

pub fn get_next(current: (isize, isize), direction: Direction) -> (isize, isize) {
    match direction {
        Direction::Up => (current.0, current.1 - 1),
        Direction::Down => (current.0, current.1 + 1),
        Direction::Left => (current.0 - 1, current.1),
        Direction::Right => (current.0 + 1, current.1),
    }
}

pub fn _print_grid(map: &Grid<Cell>) {
    for y in 0..map.rows() {
        for x in 0..map.cols() {
            if let Some(cell) = map.get(y, x) {
                print!(
                    "{}",
                    match cell {
                        Cell::Space => ".",
                        Cell::Puffy => "@",
                        Cell::Block => "#",
                        Cell::Bubble => "O",
                        Cell::Unknown => "X",
                    }
                );
            }
        }
        println!()
    }
}

pub mod part1 {
    use crate::*;
    pub fn solve(file: String) -> u64 {
        let (mut map, steps) = parse(file);
        let cols = map.cols();

        for step in steps {
            let puffy_id = map
                .iter()
                .enumerate()
                .find(|(_pos, c)| **c == Cell::Puffy)
                .expect("cannot find puffy")
                .0;
            let puffy = ((puffy_id % cols) as isize, (puffy_id / cols) as isize);
            let mut chian = Vec::<(isize, isize)>::new();
            chian.push((puffy.0, puffy.1));

            loop {
                let next = get_next(*chian.last().unwrap(), step);
                let item = map.get(next.1, next.0);
                chian.push(next);
                if item == Some(&Cell::Space) {
                    chian.windows(2).rev().for_each(|pair| {
                        map.swap(
                            (pair[0].1 as usize, pair[0].0 as usize),
                            (pair[1].1 as usize, pair[1].0 as usize),
                        );
                    });
                    break;
                } else if item == Some(&Cell::Block) {
                    break;
                }
            }
        }
        _print_grid(&map);

        map.iter()
            .enumerate()
            .filter_map(|(pos, cell)| {
                if cell == &Cell::Bubble {
                    Some(pos / cols * 100 + pos % cols)
                } else {
                    None
                }
            })
            .sum::<usize>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
