pub fn parse(file: String) -> (usize, usize) {
    let (start_str,end_str) = file.split_once("-").unwrap();

    (
        start_str.parse::<usize>().expect("cannot parse"),
        end_str.parse::<usize>().expect("cannot parse")
    )
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> usize {
        let (start, end) = parse(file);
        let mut count = 0;
        for n in start..=end {
            let v_char = format!("{:06}", n).chars().collect::<Vec<char>>();
            // checking Two adjacent digit are same (like 22 in 122345)
            if ! v_char.windows(2).any(|pair| {
                    pair[0] == pair[1]
                })
            {
                continue;
            }

            // going from left to right, digit never decrease
            if v_char.windows(2).any(|pair| {
                    pair[0] > pair[1]
                })
            {
                continue;
            }
            count += 1;
        }
        count
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
