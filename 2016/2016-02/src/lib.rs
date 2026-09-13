use grid::Grid;

pub fn solve(grid: Grid<char>, init_pos: (i32, i32), file: String) -> String {
    let mut code = String::new();
    let mut pos = init_pos;
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            line.chars().for_each(|c| {
                let new_pos = match c {
                    'U' => (pos.0 - 1, pos.1),
                    'L' => (pos.0, pos.1 - 1),
                    'D' => (pos.0 + 1, pos.1),
                    'R' => (pos.0, pos.1 + 1),
                    _ => unreachable!(),
                };
                if let Some(n) = grid.get(new_pos.0, new_pos.1)
                    && n != &' '
                {
                    pos = new_pos
                }
            });

            let n = grid.get(pos.0, pos.1).unwrap();
            code += format!("{n}").as_str();
        });

    code
}

pub mod part1 {
    use grid::grid;

    pub fn solve(file: String) -> String {
        let grid = grid![
            ['1', '2', '3']
            ['4', '5', '6']
            ['7', '8', '9']
        ];
        crate::solve(grid, (1, 1), file)
    }
}

pub mod part2 {
    use grid::grid;

    pub fn solve(file: String) -> String {
        let grid = grid![
            [' ', ' ', '1', ' ', ' ']
            [' ', '2', '3', '4', ' ']
            ['5', '6', '7', '8', '9']
            [' ', 'A', 'B', 'C', ' ']
            [' ', ' ', 'D', ' ', ' ']
        ];
        crate::solve(grid, (2, 0), file)
    }
}
