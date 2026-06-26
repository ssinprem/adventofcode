
pub mod part1 {
    use regex::Regex;
    pub fn solve(file: String) -> u64 {
        let reg_group = Regex::new(
            r"mul\((\d+),(\d+)\)"
        ).unwrap();


        reg_group.captures_iter(file.as_str())
            .map(|c| c.extract())
            .collect::<Vec<_>>()
            .iter().map(|(_full, [a,b])| {
                a.parse::<u64>().expect("cannot parse") * 
                b.parse::<u64>().expect("cannot parse")
            }).sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
