
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
    use regex::Regex;
    pub fn solve(file: String) -> u64 {
        let reg_group = Regex::new(
            r"(?:mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))"
        ).unwrap();

        let mut does = true;
        let mut sum = 0;
        reg_group.captures_iter(file.as_str())
            .for_each(|capture| {
                if let Some(a) = capture.get(1) &&
                   let Some(b) = capture.get(2) && 
                   does 
                {
                       sum += a.as_str().parse::<u64>().expect("cannot parse") * 
                              b.as_str().parse::<u64>().expect("cannot parse");
                } else if capture.get(3).is_some() { // do()
                    does = true;
                } else if capture.get(4).is_some() { // don't()
                    does = false;
                }
            });
        sum
    }
}
