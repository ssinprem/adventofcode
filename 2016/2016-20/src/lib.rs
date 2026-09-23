use std::range::RangeInclusive;

pub fn parse(file: String) -> Vec<RangeInclusive<u32>> {
    file.lines().map(|line| {
        let (start,end) = line.split_once("-").unwrap();
        let start = start.parse::<u32>().expect("cannot parse");
        let end = end.parse::<u32>().expect("cannot parse");

        RangeInclusive::<u32>::from(start..=end)
    }).collect()
}
pub mod part1 {
    use crate::parse;

    pub fn solve(file: String, max: u32) -> Option<u32> {
        let mut rules = parse(file);
        rules.sort_by_key(|range| range.start);
        let mut last = rules[0].last;
        for rule in rules.windows(2) {
            let rule1 = rule[0];
            let rule2 = rule[1];
            println!("{rule1:10?}    {rule2:10?}");
            if last > max {
                return None;
            }
            if last+1 < rule2.start {
                return Some( rule1.last + 1 );
            }
            last = rule1.last.max(rule2.last);
        }
        None
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
