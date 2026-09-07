use grid::*;

pub fn parse(file: String) -> Grid<bool> {
    let mut map = Grid::new(0, 0);
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            map.push_row(line.chars().map(|c| c == '#').collect());
        });
    map
}

pub fn _display(map: &Grid<bool>) -> String {
    let mut output = "".to_string();

    let rows = map.rows();
    let cols = map.cols();

    for y in 0..rows {
        for x in 0..cols {
            let val = map.get(y, x).unwrap();
            if *val {
                output += "#";
            } else {
                output += ".";
            }
        }
        output += "\n";
    }
    output
}

pub fn process(map: &Grid<bool>) -> Grid<bool> {
    let mut new_map = map.clone();

    let rows = map.rows();
    let cols = map.cols();

    let neighbor: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for y in 0..rows {
        for x in 0..cols {
            let light = map.get(y, x).unwrap();
            let n_cnt = neighbor
                .iter()
                .filter_map(|(dy, dx)| map.get(y as isize + dy, x as isize + dx))
                .filter(|l| **l)
                .count();

            let cell = new_map.get_mut(y, x).unwrap();
            if *light {
                *cell = [2, 3].contains(&n_cnt);
            } else {
                *cell = n_cnt == 3;
            }
        }
    }

    new_map
}

pub mod part1 {
    use crate::{parse, process};

    pub fn solve(file: String, steps: u64) -> u64 {
        let mut map = parse(file);
        for _ in 0..steps {
            map = process(&map);
        }
        map.iter().filter(|light| **light).count() as u64
    }
}

pub mod part2 {
    use crate::{parse, process};

    pub fn solve(file: String, steps: u64) -> u64 {
        let mut map = parse(file);
        let (cols, rows) = (map.cols(), map.rows());
        [(0, 0), (0, cols - 1), (rows - 1, 0), (rows - 1, cols - 1)]
            .iter()
            .for_each(|(y, x)| {
                let cornor = map.get_mut(*y, *x).unwrap();
                *cornor = true
            });

        for _i in 0..steps {
            map = process(&map);

            [(0, 0), (0, cols - 1), (rows - 1, 0), (rows - 1, cols - 1)]
                .iter()
                .for_each(|(y, x)| {
                    let cornor = map.get_mut(*y, *x).unwrap();
                    *cornor = true
                });
        }
        map.iter().filter(|light| **light).count() as u64
    }
}
