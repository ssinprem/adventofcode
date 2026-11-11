pub fn parse(file: String) -> Vec<((isize, isize, isize), usize)> {
    let regex = regex::Regex::new(r"pos=<([-0-9]+),([-0-9]+),([-0-9]+)>, r=(\d+)").unwrap();
    file.lines()
        .map(|line| {
            let caps = regex.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
            let z = caps.get(3).unwrap().as_str().parse::<isize>().unwrap();
            let r = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            ((x, y, z), r)
        })
        .collect()
}

pub fn range(bot1: &(isize, isize, isize), bot2: &(isize, isize, isize)) -> usize {
    bot1.0.abs_diff(bot2.0) + bot1.1.abs_diff(bot2.1) + bot1.2.abs_diff(bot2.2)
}

pub mod part1 {
    use crate::{parse, range};

    pub fn solve(file: String) -> usize {
        let bots = parse(file);

        let big_bot = bots.iter().max_by_key(|(_pos, range)| *range).unwrap();
        bots.iter()
            .filter(|bot| {
                let range = range(&big_bot.0, &bot.0);
                range <= big_bot.1
            })
            .count()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
