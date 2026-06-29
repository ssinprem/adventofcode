
pub fn parse(file: String) -> Vec<(u64,Vec<u64>)> {
    file.lines().filter(|line| !line.is_empty())
    .filter_map(|line| {
        if let Some((str_result,str_num)) = line.split_once(":") {
            Some((
                str_result.parse::<u64>().expect("cannot parse"),
                str_num.split_whitespace()
                    .map(|str| str.parse::<u64>().expect("cannot parse"))
                    .collect()
            ))
        } else {
            None
        }
    }).collect()
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let datas = parse(file);
        datas.iter().filter_map(|(result, nums)| {
            
            for case in 0..2_i32.pow(nums.len() as u32-1) {
                let mut sum = *nums.first().expect("cannot get first num");
                for (i, num) in nums.iter().enumerate().skip(1) {
                    if case & (1<<(i-1) as i32) != 0 {
                        sum += num;
                    } else {
                        sum *= num;
                    }
                }
                if sum == *result {
                    println!(" ✅ Valid    {result} {nums:?}");
                    return Some(result);
                }
            }
            println!(" ❌ Invalid  {result} {nums:?}");
            None
        }).sum()
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let datas = parse(file);
        datas.iter().filter_map(|(result, nums)| {
            for case in 0..3_i32.pow(nums.len() as u32 -1) {
                let mut sum = *nums.first().expect("cannot get first num");
                // println!(" #{case}  ");
                for (i, num) in nums.iter().enumerate().skip(1) {
                    // print!("----  {sum}  {}",nums[i]);
                    match (case / 3_i32.pow(i as u32-1)) % 3 {
                        0 => { sum += num; },
                        1 => { sum *= num; },
                        2 => { sum = (sum.to_string() + num.to_string().as_str())
                                    .parse::<u64>().expect("cannot parse");
                             },
                        _ => {}
                    }
                    // println!(" {sum} ")
                }
                if sum == *result {
                    println!("✅ Valid    {result} {nums:?}");
                    return Some(result);
                }
            }
            println!("❌ Invalid  {result} {nums:?}");
            None
        }).sum()
    }
}
