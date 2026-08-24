use grid::*;
use rayon::prelude::*;

pub fn parse(file: String) -> (Grid<char>, (usize, usize), (usize, usize)) {
    let mut maps: Grid<char> = Grid::new(0, 0);

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            maps.push_row(line.chars().collect());
        });

    let rows = maps.rows();
    let cols = maps.cols();
    let start = (0, 1);
    let end = (rows - 1, cols - 2);

    (maps, start, end)
}

pub mod part1 {
    use super::*;

    pub fn pathing(
        maps: &Grid<char>,
        path: Vec<(usize, usize)>,
        end: (usize, usize),
    ) -> Vec<Vec<(usize, usize)>> {
        let rows = maps.rows();
        let cols = maps.cols();
        let mut path = path.clone();
        while let Some(cur) = path.last()
            && cur != &end
        {
            let candidate = [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .iter()
                .filter_map(|(dy, dx)| {
                    let new_y = cur.0 as isize + dy;
                    let new_x = cur.1 as isize + dx;

                    if new_y < 0
                        || new_y >= rows as isize
                        || new_x < 0
                        || new_x >= cols as isize
                        || path.contains(&(new_y as usize, new_x as usize))
                    {
                        None
                    } else if let Some(block) = maps.get(new_y, new_x) {
                        if block == &'.'
                            || (block == &'^' && (dy, dx) == (&-1, &0))
                            || (block == &'v' && (dy, dx) == (&1, &0))
                            || (block == &'<' && (dy, dx) == (&0, &-1))
                            || (block == &'>' && (dy, dx) == (&0, &1))
                        {
                            Some((new_y as usize, new_x as usize))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if candidate.len() == 1 {
                path.push(*candidate.last().unwrap());
            } else if candidate.len() > 1 {
                return candidate
                    .par_iter()
                    .flat_map(|can| {
                        let mut path = path.clone();
                        path.push(*can);
                        pathing(maps, path, end)
                    })
                    .collect();
            } else {
                return vec![];
            }
        }
        vec![path]
    }

    pub fn solve(file: String) -> u64 {
        let (maps, start, end) = parse(file);
        let paths = pathing(&maps, vec![start], end);
        paths
            .iter()
            // .inspect(|p| println!("{}", p.len()))
            .max_by_key(|p| p.len())
            .unwrap()
            .len() as u64
            - 1
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
