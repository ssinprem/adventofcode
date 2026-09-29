pub mod part1 {
    pub fn solve(file: String) -> u32 {
        file.lines()
            .map(|line| {
                let list = line
                    .split_whitespace()
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let min = *list.iter().min().unwrap();
                let max = *list.iter().max().unwrap();
                max - min
            })
            .sum()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
