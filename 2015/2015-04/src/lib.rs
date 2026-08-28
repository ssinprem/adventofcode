pub mod part1 {
    pub fn solve(key: String) -> u64 {
        (1..).find(|n| {
            let value = format!("{key}{n}");
            let hash = format!("{:x}",md5::compute(value));
            hash.starts_with("00000")
        }).unwrap() as u64
    }
}

pub mod part2 {
    pub fn solve(_key: String) -> u64 {
        0
    }
}
