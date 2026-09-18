use grid::Grid;

pub fn parse(file: String) -> (usize, usize, usize) {
    let val = file
        .split_terminator(&[' ', ','])
        .map(|str| str.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (val[0], val[1], val[2])
}

pub fn generate_map(fev: usize, max_x: usize, max_y: usize) -> Grid<bool> {
    fn cal(x: usize, y: usize) -> usize {
        x * x + 3 * x + 2 * x * y + y + y * y
    }
    let mut maps = Grid::new(max_y + 1, max_x + 1);

    for y in 0..=max_y {
        for x in 0..=max_x {
            let cal = cal(x, y);
            let even = format!("{:b}", cal + fev)
                .chars()
                .filter(|char| char == &'1')
                .count()
                .is_multiple_of(2);

            let cell = maps.get_mut(y, x).unwrap();
            *cell = even;
        }
    }
    maps
}

pub fn _display(maps: &Grid<bool>, paths: &[(usize, usize)]) -> String {
    let mut output = String::new();
    let rows = maps.rows();
    let cols = maps.cols();

    for row in 0..rows {
        for col in 0..cols {
            let cel = maps.get(row, col).unwrap();
            output += if paths.contains(&(col, row)) {
                "O"
            } else if *cel {
                "."
            } else {
                "#"
            };
        }
        output += "\n";
    }
    output
}

pub fn dijkstra(
    maps: &Grid<bool>,
    start: (usize, usize),
    end: (usize, usize),
) -> (Vec<(usize, usize)>, usize) {
    use pathfinding::prelude::dijkstra;
    #[derive(PartialEq, Eq, Hash, Clone)]
    struct Pos(usize, usize);
    let (vec, count) = dijkstra(
        &Pos(start.0, start.1),
        |p| {
            let &Pos(x, y) = p;
            [(-1, 0), (1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter_map(|(dx, dy)| {
                    let next_y = (y as isize + dy) as usize;
                    let next_x = (x as isize + dx) as usize;
                    if let Some(cell) = maps.get(next_y, next_x)
                        && *cell
                    {
                        Some((Pos(next_x, next_y), 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(Pos, usize)>>()
        },
        |p| p == &Pos(end.0, end.1),
    )
    .unwrap();

    let vec = vec
        .into_iter()
        .map(|p| (p.0, p.1))
        .collect::<Vec<(usize, usize)>>();
    (vec, count)
}

pub mod part1 {
    use crate::{_display, dijkstra, generate_map, parse};

    pub fn solve(file: String) -> u64 {
        let (fev, x, y) = parse(file);
        let maps = generate_map(fev, x * 3 / 2, y * 3 / 2);
        let (path, cnt) = dijkstra(&maps, (1, 1), (x, y));
        println!("{}", _display(&maps, &path));
        cnt as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
