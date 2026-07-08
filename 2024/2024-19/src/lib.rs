use std::collections::HashMap;

pub fn parse(file: String) -> (Vec<String>, Vec<String>) {
    let (str_mat, str_order) = file.split_once("\n\n").unwrap();
    (
        str_mat
            .split(",")
            .map(|str| str.trim().to_string())
            .collect(),
        str_order
            .lines()
            .map(|str| str.trim().to_string())
            .collect(),
    )
}

pub fn makable(order: String, pattern: &Vec<String>, lib: &mut HashMap<String, u64>) -> u64 {
    if order.is_empty() {
        return 1;
    }

    if let Some(cache) = lib.get(&order.to_string()) {
        *cache
    } else {
        let score = pattern
            .iter()
            .filter_map(|pat| {
                if order.starts_with(pat) {
                    Some(makable(
                        order.strip_prefix(pat).unwrap().to_string(),
                        pattern,
                        lib,
                    ))
                } else {
                    None
                }
            })
            .sum();
        lib.insert(order.to_string(), score);
        score
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (pattern, orders) = parse(file);
        let mut lib = HashMap::<String, u64>::new();
        orders
            .iter()
            .filter(|order| makable(order.to_string(), &pattern, &mut lib) > 0)
            .inspect(|order| println!("✅ {order}"))
            .count() as u64
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (pattern, orders) = parse(file);
        let mut lib = HashMap::<String, u64>::new();
        orders
            .iter()
            .map(|order| (order, makable(order.to_string(), &pattern, &mut lib)))
            .inspect(|(order, option)| {
                if *option > 0 {
                    println!("✅ {order} : {option}");
                } else {
                    println!("❌ {order}");
                }
            })
            .map(|(_, option)| option)
            .sum::<u64>()
    }
}
