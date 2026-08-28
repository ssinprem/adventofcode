pub mod part1 {
    pub fn is_valid(text: String) -> bool {
        text.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
        &&
        text.chars().collect::<Vec<char>>()
            .windows(2).any(|cc| cc[0] == cc[1])
        &&
        !["ab","cd","pq","xy"].iter().any(|ban| text.contains(ban))
    }

    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .filter(|line| is_valid(line.to_string()))
        .count() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
