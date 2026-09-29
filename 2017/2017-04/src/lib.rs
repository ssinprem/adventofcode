pub mod part1 {
    pub fn solve(file: String) -> usize {
        file.lines()
            .filter(|line| {
                let mut words = line
                    .split_whitespace()
                    .map(|str| str.to_string())
                    .collect::<Vec<String>>();
                words.sort();
                words.windows(2).all(|pairs| pairs[0] != pairs[1])
            })
            .count()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
