use grid::*;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Cell {
    #[default]
    Unknown,
    Wall,
    Path,
    Start,
    End,
}

pub fn parse(file: String) -> Grid<Cell> {
    let cols = file.lines().find(|line| !line.is_empty()).iter().len();
    let mut map = Grid::new(0, cols);

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            map.push_row(
                line.chars()
                    .map(|c| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Path,
                        'S' => Cell::Start,
                        'E' => Cell::End,
                        _ => Cell::Unknown,
                    })
                    .collect(),
            );
        });
    map
}

pub fn _print_map(map: &Grid<Cell>, path: &[(isize, isize)]) {
    let cols = map.cols();
    let rows = map.rows();
    for y in 0..rows {
        for x in 0..cols {
            print!(
                "{}",
                match (
                    map.get(y, x),
                    path.iter()
                        .any(|(px, py)| *py == y as isize && *px == x as isize)
                ) {
                    (Some(Cell::Start), _) => "S",
                    (Some(Cell::End), _) => "E",
                    (Some(Cell::Path), false) => ".",
                    (Some(Cell::Path), true) => "O",
                    (Some(Cell::Wall), _) => "#",
                    _ => "?",
                }
            );
        }
        println!();
    }
}

pub mod part1 {
    use super::*;
    use std::collections::HashMap;

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        // _print_map(&map, &[]);
        let cols = map.cols() as isize;
        let start_idx = map
            .iter()
            .clone()
            .enumerate()
            .find(|(_id, cell)| cell == &&Cell::Start)
            .expect("cannot find start")
            .0 as isize;
        let end_idx = map
            .iter()
            .clone()
            .enumerate()
            .find(|(_id, cell)| cell == &&Cell::End)
            .expect("cannot find end")
            .0 as isize;
        let start = (start_idx % cols, start_idx / cols);
        let end = (end_idx % cols, end_idx / cols);
        #[allow(clippy::type_complexity)]
        let mut scores: HashMap<(isize, isize), (u64, Option<(isize, isize)>)> = (0..map.rows())
            .flat_map(|x| {
                (0..map.cols())
                    .filter_map(|y| {
                        if let Some(cell) = map.get(y, x)
                            && cell != &Cell::Wall
                        {
                            Some(((x as isize, y as isize), (u64::MAX, None)))
                        } else {
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>()
            })
            .collect::<HashMap<_, _>>();
        let mut stack = Vec::<((isize, isize), (isize, isize), u64)>::new();
        // init start point with score 0
        scores.insert(start, (0, None));
        stack.push((start, (0, 0), 0));

        while let Some((pos, dir, score)) = stack.pop() {
            if score
                > scores
                    .get(&pos)
                    .expect("cannot get score at current vertex")
                    .0
            {
                continue;
            }
            for next_dir in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                // ignore backward directions
                if next_dir.0 == -dir.0 && next_dir.1 == -dir.1 {
                    continue;
                }

                let next_pos = (pos.0 + next_dir.0, pos.1 + next_dir.1);
                // ignore wall
                if Some(&Cell::Wall) == map.get(next_pos.1, next_pos.0) {
                    continue;
                }

                let new_score = if dir != next_dir {
                    1000 + 1 + score
                } else {
                    1 + score
                };
                if new_score
                    < scores
                        .get(&next_pos)
                        .expect("cannot get score at next_pos")
                        .0
                {
                    scores.insert(next_pos, (new_score, Some(pos)));
                    stack.push((next_pos, next_dir, new_score));
                }
            }
        }

        scores.get(&end).expect("cannot get end point").0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
