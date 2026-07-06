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

    fn score(path: &Vec<(isize,isize)>) -> u64 {
        let turn = path
                    .windows(3)
                    .filter(|row| row[0].0 != row[2].0 && row[0].1 != row[2].1)
                    .count()
                    + 1;
        (turn * 1000 + path.len() - 1) as u64
    }
    fn find_path(
        map: &Grid<Cell>,
        path: Vec<(isize, isize)>,
        target: (isize, isize),
        best: &mut u64
    ) -> Option<Vec<Vec<(isize, isize)>>> {
        
        let curr = path.last().expect("cannot get current cell");
        // _print_map(map, &path);
        if curr == &target {
            // _print_map(map, &path);
            *best = score(&path).min(*best);
            println!("new best score :{best}");
            return Some(vec![path]);
        }

        let paths: Vec<Vec<(isize, isize)>> = ([(-1, 0), (0, -1), (1, 0), (0, 1)])
            .iter()
            .filter_map(|direction| {
                let next = (curr.0 + direction.0, curr.1 + direction.1);
                // ignore the old line
                if path.contains(&next) {
                    return None;
                }
                // ignore wall
                if let Some(item) = map.get(next.1, next.0)
                    && item == &Cell::Wall
                {
                    return None;
                }
                let mut next_path = path.clone();
                next_path.push(next);
                // ignore if score worse then the best score.
                if *best != u64::MAX && score(&next_path) >= *best {
                    return None;
                }
                find_path(map, next_path, target, best)
            })
            .flatten()
            .collect();

        if paths.is_empty() { None } else { Some(paths) }
    }

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        _print_map(&map, &[]);
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
            .expect("cannot find start")
            .0 as isize;
        let start = (start_idx % cols, start_idx / cols);
        let end = (end_idx % cols, end_idx / cols);
        let mut best = u64::MAX;
        let paths = find_path(&map, vec![start], end, &mut best).expect("cannot find path");
        let scores: Vec<_> = paths
            .iter()
            .map(|path| {
                (score(path), path)
            })
            .collect();

        scores
            .iter()
            .min_by_key(|s| s.0)
            .inspect(|min| _print_map(&map, min.1))
            .expect("cannot find min score")
            .0 as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
