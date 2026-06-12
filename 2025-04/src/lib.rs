pub const ADJ_OFFSET : [(isize, isize); 8] =
[(-1,-1), (-1, 0), (-1, 1),
 ( 0,-1),          ( 0, 1),
 ( 1,-1), ( 1, 0), ( 1, 1)
];

pub fn edit_string(string: String, x: usize, y: usize, ch: char) -> String {
    string.clone().lines().enumerate()
        .map(|(gy, line)| {
            if gy == y {
                line.chars().enumerate()
                    .map(|(gx, char)| {
                        if gx == x {
                            ch
                        } else {
                            char
                        }
                    })
                    .collect::<String>()
            } else {
                line.to_string()
            }
        }).map(|line| line+"\n").collect()
}

pub mod part1 {
    pub fn solve(file: String) -> u64 {
        let mut count = 0;
        let grid = file.clone();
        let mut next_grid = grid.clone();
        for (y,line) in grid.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '@' {
                    let adj_rolls = crate::ADJ_OFFSET.map(|(dy,dx)| {
                        if let Some(mut x) = grid.lines().nth((y as isize +dy) as usize)
                            .and_then(|nline| nline.chars().nth((x as isize +dx) as usize)) {
                            if x == '@' {
                                1
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    }).iter().sum::<u64>();
                    if adj_rolls < 4 {
                        next_grid = crate::edit_string(next_grid, x, y, 'x');
                        count += 1;
                    }
                }
            }
        }
        println!("{next_grid} => {count}");
        count
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        let mut grid = file.clone();
        let (mut cnt, mut pre_cnt) = (0,1);
        while pre_cnt != cnt {
            pre_cnt = cnt;
            let mut next_grid = grid.clone();
            for (y, line) in grid.lines().enumerate() {
                for (x, char) in line.chars().enumerate() {
                    if char == '@' {
                        let adj_rolls = crate::ADJ_OFFSET.map(|(dy,dx)| {
                            if let Some(x) = grid.lines().nth((y as isize +dy) as usize)
                                .and_then(|nline| nline.chars().nth((x as isize +dx) as usize)) {
                                if x == '@' {
                                    1
                                } else {
                                    0
                                }
                            } else {
                                0
                            }
                        }).iter().sum::<u64>();
                        if adj_rolls < 4 {
                            next_grid = crate::edit_string(next_grid.clone(), x, y, 'x');
                            cnt += 1;
                        }
                    }
                }
            }
            println!("{next_grid} => {pre_cnt} + {} = {cnt}", cnt-pre_cnt);
            grid = next_grid.replace("x", ".");
        }
        cnt
    }
}
