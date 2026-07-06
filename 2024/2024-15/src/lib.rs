use grid::*;

#[derive(Default, Debug, PartialEq)]
pub enum Cell {
    #[default]
    Unknown,
    Space,
    Box,
    Block,
    Puffy,
    BoxL,
    BoxR,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
                        Cell::Box => "O",
                        Cell::BoxL => "[",
                        Cell::BoxR => "]",
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
                            'O' => Cell::Box,
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
        // _print_grid(&map);

        map.iter()
            .enumerate()
            .filter_map(|(pos, cell)| {
                if cell == &Cell::Box {
                    Some(pos / cols * 100 + pos % cols)
                } else {
                    None
                }
            })
            .sum::<usize>() as u64
    }
}

pub mod part2 {
    use super::*;

    use std::io;
    use std::io::prelude::*;
    fn _pause() {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
        write!(stdout, "Press any key to continue...").unwrap();
        stdout.flush().unwrap();

        // Read a single byte and discard
        let _ = stdin.read(&mut [0u8]).unwrap();
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
                        .flat_map(|c| match c {
                            '@' => [Cell::Puffy, Cell::Space],
                            '.' => [Cell::Space, Cell::Space],
                            '#' => [Cell::Block, Cell::Block],
                            'O' => [Cell::BoxL, Cell::BoxR],
                            _ => [Cell::Unknown, Cell::Unknown],
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

    pub fn solve(file: String) -> u64 {
        let (mut map, steps) = parse(file);
        let cols = map.cols();
        // _print_grid(&map);
        for step in steps {
            // println!("{step:?}");
            let puffy_id = map
                .iter()
                .enumerate()
                .find(|(_pos, c)| **c == Cell::Puffy)
                .expect("cannot find puffy")
                .0;
            let puffy = ((puffy_id % cols) as isize, (puffy_id / cols) as isize);
            let mut chian = Vec::<Vec::<(isize, isize)>>::new();
            chian.push(vec![(puffy.0, puffy.1)]);

            loop {
                let mut nexts: Vec<(isize, isize)> = chian.last_mut().unwrap().iter()
                    .filter_map(|next| {
                        let current_item = map.get(next.1, next.0);
                        // if current cell is Space, no need check next cell
                        if current_item == Some(&Cell::Space) {
                            None
                        } else {
                            Some(get_next(*next, step))
                        }
                    })
                    .collect();
                
                for n in nexts.clone() {
                    let item = map.get(n.1, n.0);
                    if step==Direction::Up || step==Direction::Down {
                        if item == Some(&Cell::BoxL) {
                            nexts.push((n.0+1,n.1));
                        }
                        if item == Some(&Cell::BoxR) {
                            nexts.push((n.0-1,n.1));
                        }
                    }
                }
                nexts.sort_by_key(|(y,x)| *y*1000+*x );
                nexts.dedup_by_key(|(y,x)| *y*1000+*x);
                
                let items : Vec<_>= nexts.iter().map(|next| {
                    (next.clone(), map.get(next.1, next.0))
                }).collect();
                // println!("{nexts:?} {items:?}");
                
                chian.push(nexts);
                if items.iter().all(|(_pos, i)| *i==Some(&Cell::Space)) {
                    // println!("Pass");

                    // Swapping
                    chian.windows(2).rev().for_each(|rows| {
                        //println!("{:?}",rows);
                        
                        let pairs = rows[1].iter().filter_map(|b| {
                            if let Some(c) = rows[0].iter().find(|c| {
                                if step == Direction::Up || step == Direction::Down {
                                    b.0 == c.0
                                } else {
                                    b.1 == c.1
                                }
                            }){
                                Some([b, c])
                            } else {
                                None
                            }
                            
                        });

                        for pair in pairs {
                            map.swap(
                                (pair[0].1 as usize, pair[0].0 as usize),
                                (pair[1].1 as usize, pair[1].0 as usize),
                            );
                        }
                    });
                    break;
                } else if items.iter().any(|(_pos, i)| *i==Some(&Cell::Block)) {
                    // println!("Block");
                    break;
                }
            }
            // _print_grid(&map);
            // _pause();
        }

        map.iter()
            .enumerate()
            .filter_map(|(pos, cell)| {
                if cell == &Cell::BoxL {
                    Some(pos / cols * 100 + pos % cols)
                } else {
                    None
                }
            })
            .sum::<usize>() as u64
    }
}
