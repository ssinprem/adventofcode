use grid::Grid;

use crate::{
    Dir::{East, North, South, West},
    Next::{Forward, Left, Right},
    Track::{Backslash, Cross, Horizontal, Slash, Space, Vertical},
};

#[derive(Debug, Default)]
pub enum Track {
    Horizontal, // -   East West
    Vertical,   // |   North South
    Slash,      // /   North West or South East
    Backslash,  // \   North East or South West
    Cross,      // +
    #[default]
    Space, // ' '
}

#[derive(Debug, Clone)]
pub enum Dir {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone)]
pub enum Next {
    Left,
    Forward,
    Right,
}

#[allow(clippy::type_complexity)]
pub fn parse(file: String) -> (Grid<Track>, Vec<(usize, usize, Dir, Next)>) {
    use Dir::*;
    use Next::Left;
    use Track::*;

    let mut grid = Grid::new(0, 0);
    let mut carts = Vec::new();

    file.lines().enumerate().for_each(|(y, line)| {
        grid.push_row(
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    match char {
                        '^' => carts.push((x, y, North, Left)),
                        'v' => carts.push((x, y, South, Left)),
                        '<' => carts.push((x, y, West, Left)),
                        '>' => carts.push((x, y, East, Left)),
                        _ => {}
                    };

                    match char {
                        '-' | '<' | '>' => Horizontal,
                        '|' | '^' | 'v' => Vertical,
                        '/' => Slash,
                        '\\' => Backslash,
                        '+' => Cross,
                        ' ' => Space,
                        _ => {
                            println!("Error  {char}");
                            unreachable!()
                        }
                    }
                })
                .collect(),
        );
    });

    (grid, carts)
}

pub fn turn(dir: &Dir, track: &Track, next: &Next) -> (Dir, Next) {
    let cross_next = match next {
        Left => Forward,
        Forward => Right,
        Right => Left,
    };
    let dir = dir.clone();
    let next = next.clone();
    let cross_dir = match (&dir, &next) {
        (current, Forward) => current.clone(),
        (North, Left) => West,
        (North, Right) => East,
        (South, Left) => East,
        (South, Right) => West,
        (West, Left) => South,
        (West, Right) => North,
        (East, Left) => North,
        (East, Right) => South,
    };

    match (&dir, &track) {
        (North, Slash) => (East, next),
        (North, Backslash) => (West, next),
        (North, Vertical) => (North, next),
        (South, Slash) => (West, next),
        (South, Backslash) => (East, next),
        (South, Vertical) => (South, next),
        (West, Slash) => (South, next),
        (West, Backslash) => (North, next),
        (West, Horizontal) => (West, next),
        (East, Slash) => (North, next),
        (East, Backslash) => (South, next),
        (East, Horizontal) => (East, next),
        (_, Cross) => (cross_dir, cross_next),
        _ => {
            println!("Error : {dir:?}  {track:?}");
            unreachable!()
        }
    }
}

pub fn tick(
    grid: &Grid<Track>,
    carts: Vec<(usize, usize, Dir, Next)>,
) -> Vec<(usize, usize, Dir, Next)> {
    carts
        .into_iter()
        .map(|(x, y, dir, next)| {
            let next_cel = match dir {
                North => (x, y - 1),
                South => (x, y + 1),
                East => (x + 1, y),
                West => (x - 1, y),
            };
            let next_track = grid.get(next_cel.1, next_cel.0).unwrap();
            let (dir, next) = turn(&dir, next_track, &next);
            (next_cel.0, next_cel.1, dir, next)
        })
        .collect()
}

pub fn check_crash(
    carts: &[(usize, usize, Dir, Next)],
    last_carts: &[(usize, usize, Dir, Next)],
) -> Vec<(usize, usize)> {
    use itertools::Itertools;
    let mut pairs = Vec::new();
    for pair in carts.iter().enumerate().combinations(2) {
        let cart_a = pair[0].1;
        let cart_b = pair[1].1;

        if cart_a.0 == cart_b.0 && cart_a.1 == cart_b.1 {
            pairs.push((pair[0].0, pair[1].0));
        }
    }

    for (a_id, cart_a) in last_carts.iter().enumerate() {
        for (b_id, cart_b) in carts.iter().enumerate() {
            if cart_a.0 == cart_b.0 && cart_a.1 == cart_b.1 
            && !pairs.contains(&(a_id, b_id)) || !pairs.contains(&(b_id, a_id)) {
                pairs.push((a_id, b_id));
            }
        }
    }

    pairs
}

pub fn _display(grid: &Grid<Track>, carts: &[(usize, usize, Dir, Next)]) {
    let cols = grid.cols();
    let rows = grid.rows();

    for y in 0..rows {
        for x in 0..cols {
            let c = carts
                .iter()
                .filter(|(cx, cy, _, _)| cx == &x && cy == &y)
                .collect::<Vec<_>>();

            if c.len() >= 2 {
                print!("X");
            } else if c.len() == 1 {
                print!(
                    "{}",
                    match c[0].2 {
                        North => "^",
                        South => "v",
                        East => ">",
                        West => "<",
                    }
                )
            } else {
                print!(
                    "{}",
                    match grid.get(y, x).unwrap() {
                        Horizontal => "-",
                        Vertical => "|",
                        Slash => "/",
                        Backslash => "\\",
                        Cross => "+",
                        Space => " ",
                    }
                );
            }
        }
        println!()
    }
}

pub mod part1 {
    use crate::{_display, check_crash, parse, tick};
    pub fn solve(file: String) -> (usize, usize) {
        let (grid, mut carts) = parse(file);
        for _ in 0.. {
            let last_carts = carts.clone();
            carts = tick(&grid, carts);
            if let pairs = check_crash(&carts, &last_carts)
                && !pairs.is_empty()
            {
                _display(&grid, &carts);
                let target_a = carts.get(pairs.first().unwrap().0).unwrap();
                let target_b = carts.get(pairs.first().unwrap().1).unwrap();
                if target_a.0 < target_b.0 || target_a.1 < target_b.1 {
                    return (target_a.0, target_a.1);
                } else {
                    return (target_b.0, target_b.1);
                }
            }
        }
        (0, 0)
    }
}

pub mod part2 {
    use crate::{_display, check_crash, parse, tick};
    pub fn solve(file: String) -> (usize, usize) {
        let (grid, mut carts) = parse(file);
        for _ in 0.. {
            if carts.len() <= 1 {
                break;
            }
            let last_carts = carts.clone();
            carts = tick(&grid, carts);
            if let pairs = check_crash(&carts, &last_carts)
                && ! pairs.is_empty()
            {
                let mut remove = pairs
                    .iter()
                    .flat_map(|pair| [pair.0, pair.1])
                    .collect::<Vec<usize>>();
                remove.sort();
                while let Some(id) = remove.pop() {
                    carts.remove(id);
                }
            }
        }
        _display(&grid, &carts);
        if let Some(last_cart) = carts.pop() {
            (last_cart.0, last_cart.1)
        } else {
            (0, 0)
        }
    }
}
