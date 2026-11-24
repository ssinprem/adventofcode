pub mod part1 {
    pub fn solve(file: String) -> usize {
        let output = aoc_2019_09::part1::solve(file);
        let map = output
            .split(",")
            .map(|n| n.parse::<u8>().unwrap() as char)
            .collect::<String>();
        let mut grid = grid::Grid::new(0, 0);

        map.lines()
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                grid.push_row(line.chars().map(|char| !matches!(char, '.')).collect())
            });
        let rows = grid.rows();
        let cols = grid.cols();

        (1..rows - 1)
            .map(|row| {
                (1..cols - 1)
                    .map(|col| {
                        if [(-1, 0), (1, 0), (0, 0), (0, 1), (0, -1)]
                            .iter()
                            .all(|(dx, dy)| {
                                *grid.get(row as isize + dy, col as isize + dx).unwrap()
                            })
                        {
                            row * col
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
