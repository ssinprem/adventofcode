
pub mod part1 {
    use regex::Regex;

    pub fn solve(file: String) -> u64 {
        let mut grid = vec![vec![false; 1000 ];1000];
        let regex = Regex::new(r"(.*)\s+(\d+),(\d+)\s+through\s+(\d+),(\d+)").unwrap();

        file.lines().for_each(|line| {
            if let Some(cap) = regex.captures(line)
            && let Some(x1) = cap.get(2)
            && let Ok(x1) = x1.as_str().parse::<usize>()
            && let Some(y1) = cap.get(3)
            && let Ok(y1) = y1.as_str().parse::<usize>()
            && let Some(x2) = cap.get(4)
            && let Ok(x2) = x2.as_str().parse::<usize>()
            && let Some(y2) = cap.get(5)
            && let Ok(y2) = y2.as_str().parse::<usize>()
            && let Some(inst) = cap.get(1)
            && let inst = inst.as_str()
            && ["toggle","turn off","turn on"].contains(&inst)
            {
                (x1..=x2).for_each(|x| {
                    (y1..=y2).for_each(|y| {
                        match inst {
                            "toggle" => grid[x][y] ^= true,
                            "turn off" => grid[x][y] = false,
                            "turn on" => grid[x][y] = true,
                            _ => {}
                        }
                        
                    })
                })
            } 
        });

        grid.iter().map(|row| 
            row.iter().filter(|c| **c).count() as u64)
        .sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
