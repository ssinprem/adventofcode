use std::collections::HashSet;
use regex::*;

pub fn parse(file: String) -> (HashSet<(u32,u32)>,Vec<Vec<u32>>) {
    let re_pair = Regex::new(
        r"(\d+)\|(\d+)"
    ).expect("Regex invalid pattern failed");

    let re_order = Regex::new(
        r"((?:\d+)(?:,(?:\d+))+)"
    ).expect("Regex invalid pattern failed");

    (
        re_pair.captures_iter(&file)
        .map(|c| c.extract())
        .map(|(_full, [a,b]) | {
            (
                a.parse().expect("cannot parse digit"),
                b.parse().expect("cannot parse digit")
            )
        }).collect::<HashSet<(u32,u32)>>(),
        re_order.captures_iter(&file)
        .map(|c| c.extract())
        .map(|(_full, [group])| {
            group.split(",")
                .map(|str| str.parse::<u32>().expect("cannot parse digit"))
            .collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>()
    )
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u32 {
        let (pairs, orders) = parse(file);
        orders.into_iter().map(|order| {
            let len = order.len();
            
            println!("{order:<3?}  ");
            for i in 0..len {
                for j in i+1..len {
                    if pairs.contains(&(order[j],order[i])) {
                        println!(" > ❌ Found {}|{}",order[j],order[i]);
                        return 0;
                    }
                }
            }
            
            let middle = *order.get((len-1)/2)
                .expect("cannot get middle page");
            println!(" > ✅ Valid  get '{middle}'");
            middle
            
        }).sum()
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u32 {
        let (pairs, orders) = parse(file);
        orders.into_iter().filter_map(|mut order| {
            let len = order.len();
            let mut full_valid = true;
            println!("{order:<3?}  ");
            for i in 0..len {
                for j in i+1..len {
                    if pairs.contains(&(order[j],order[i])) {
                        println!(" > ❌ Found {}|{}",order[j],order[i]);
                        full_valid = false;
                        order.swap(j, i);
                        println!(" > Swap to {order:?}");
                    }
                }
            }
            if full_valid {
                println!(" > ✅✅ Valid at first ignore result");
                None
            } else {
                let middle = *order.get((len-1)/2)
                    .expect("cannot get middle page");
                println!(" > ✅ fix to Valid  get '{middle}'");
                Some(middle)
            }
        }).sum()
    }
}
