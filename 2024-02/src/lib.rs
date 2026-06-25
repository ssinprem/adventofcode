pub fn parse(file: String) -> Vec<Vec<i32>> {
    file.lines().filter(|line| ! line.is_empty())
    .map(|line| 
    {
        line.split_whitespace()
        .map(|str| str.parse::<i32>().expect("cannot parse number"))
        .collect::<Vec<i32>>()
    }).collect()
}

pub fn validate_list(list: &[i32]) -> bool {
    let mut pairs = list.windows(2);
                
    // find direction
    let pair = pairs.next().expect("cannot take first pair");
    let range: std::ops::RangeInclusive<i32> = 
        match pair[1] - pair[0] {
            1..=3  => 1..=3_i32,
            -3..=-1 => -3..=-1,
            _ => {
                return false;
            }
        };
    
    pairs.into_iter().all(|pair| {
        matches!(pair[1] - pair[0], diff if range.contains(&diff))
        // match pair[1] - pair[0] {
        //     diff if range.contains(&diff) => true,
        //     _ => {
        //         false
        //     }
        // }
    })
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let lists = parse(file);
        lists.iter().filter(|&list| validate_list(list))
        .count() as u64
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let lists = parse(file);
        lists.iter().filter(|&list|
            (-1..list.len() as isize).map(|rem| {
                let mut new_list = list.clone();
                if rem < 0 {
                    new_list
                } else {
                    new_list.remove(rem as usize);
                    new_list
                }
            }).any(|new_list| validate_list(&new_list))
        )
        .count() as u64
    }
}
