pub mod part1 {
    use regex::Regex;
    pub fn solve(file: String) -> i64 {
        let regex = Regex::new(r"[-0-9]+").unwrap();

        let mut sum = 0;
        for cap in regex.captures_iter(&file) {
            if let Ok(n) = cap.get_match().as_str().parse::<i64>() {
                sum += n;
            }
        }

        sum
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
