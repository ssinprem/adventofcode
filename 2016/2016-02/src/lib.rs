pub mod part1 {
    use grid::Grid;

    pub fn solve(file: String) -> String {
        let mut code = String::new();
        let mut grid = Grid::new(0,0);
        grid.push_row(vec![1,2,3]);
        grid.push_row(vec![4,5,6]);
        grid.push_row(vec![7,8,9]);
        let mut pos = (1,1);

        file.lines().filter(|line| !line.is_empty())
        .for_each(|line| {
            line.chars().for_each(|c| {
                match c {
                    'U' => pos.0 -= 1,
                    'L' => pos.1 -= 1,
                    'D' => pos.0 += 1,
                    'R' => pos.1 += 1,
                    _ => {}
                }
                pos.0 = pos.0.max(0).min(2);
                pos.1 = pos.1.max(0).min(2);
            });

            let n = grid.get(pos.0, pos.1).unwrap();
            code += format!("{n}").as_str();
        });

        code
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
