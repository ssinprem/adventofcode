pub mod part1 {
    pub fn solve(file: String) -> u64 {
        file.lines().enumerate().filter(|&(_, line)| !line.is_empty())
            .filter(|&(idx, line)| {
                let list = line.split_whitespace()
                .map(|str| str.parse::<i32>().expect("cannot parse number"))
                .collect::<Vec<i32>>();
                let mut pairs = list.windows(2);
                
                // find direction
                let pair = pairs.next().expect("cannot take first pair");
                let range: std::ops::RangeInclusive<i32> = 
                    match pair[1] - pair[0] {
                        1..=3  => 1..=3_i32,
                        -3..=-1 => -3..=-1,
                        _ => {
                            println!("{:4}. found error {:3?} {:2} -> {line}", idx+1, pair, pair[1] - pair[0] );
                            return false;
                        }
                    };
                
                pairs.into_iter().all(|pair| {
                    // matches!(pair[0] - pair[1], -2..3)
                    match pair[1] - pair[0] {
                        diff if range.contains(&diff) => true,
                        diff => {
                            println!("{:4}. found error {:3?} {:2} -> {line}", idx+1, pair, diff);
                            false
                        }
                    }
                })
            })
        .count() as u64
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        file.lines().filter(|&line| !line.is_empty())
        .enumerate().filter(|&(idx, line)| {
            let list = line.split_whitespace()
                .map(|str| str.parse::<i32>().expect("cannot parse number"))
                .collect::<Vec<i32>>();

            (-1..list.len() as isize).map(|rem| {
                let mut new_list = list.clone();
                if rem < 0  {
                    new_list
                } else {
                    new_list.remove(rem as usize);
                    new_list.clone()
                }
            }).any(|new_list| {
                println!("{:4}. {new_list:?}", idx+1);
                let mut pairs = new_list.windows(2);
    
                // find direction
                let pair = pairs.next().expect("cannot take first pair");
                let range: std::ops::RangeInclusive<i32> = 
                    match pair[1] - pair[0] {
                        1..=3  => 1..=3_i32,
                        -3..=-1 => -3..=-1,
                        _ => {
                            println!("{:4}. found error {:3?} {:2} -> {line}", idx+1, pair, pair[1] - pair[0] );
                            return false;
                        }
                    };
                
                pairs.into_iter().all(|pair| {
                    match pair[1] - pair[0] {
                        diff if ! range.contains(&diff) => {
                            println!("{:4}. found error {:3?} {:2} -> {line}", idx+1, pair, diff);
                            return false;
                        }
                        _ => {}
                    }
                    true
                })
            })
        })
        .count() as u64
    }
}
