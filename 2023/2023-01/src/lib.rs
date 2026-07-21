pub mod part1 {
    pub fn solve(file: String) -> u64 {
        file.lines()
            .filter_map(|line| {
                if let Some(a) = line.find(|c: char| c.is_ascii_digit())
                    && let Some(na) = line.chars().nth(a).and_then(|n| n.to_digit(10))
                    && let Some(b) = line.rfind(|c: char| c.is_ascii_digit())
                    && let Some(nb) = line.chars().nth(b).and_then(|n| n.to_digit(10))
                {
                    Some(na * 10 + nb)
                } else {
                    None
                }
            })
            .sum::<u32>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
