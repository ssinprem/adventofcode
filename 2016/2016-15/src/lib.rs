pub fn parse(file: String) -> Vec<(usize,usize)>{
    use regex::Regex;

    let regex = Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
    file.lines().map(|line| {
        let caps = regex.captures(line).expect("cannot regex");
        (
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap()
        )
    }).collect()
    
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let discs = parse(file);
        let mut time=0;
        loop {
            if discs.iter().enumerate().all(|(t,&(max,start))| {
                (time + start + t + 1 ) % max == 0
            }) {
                return time as u64;
            }
            time += 1;
        }
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
