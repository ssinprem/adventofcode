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

pub mod part1 {
    use super::*;

    pub fn makable(order: String, pattern: &Vec<String>) -> bool {
        if order.is_empty() {
            return true;
        }

        pattern.iter().any(|pat| {
            if order.starts_with(pat) {
                makable(order.strip_prefix(pat).unwrap().to_string(), pattern)
            } else {
                false
            }
        })
    }

    pub fn solve(file: String) -> u64 {
        let (pattern, orders) = parse(file);

        println!("{pattern:?}");
        println!("{orders:?}");

        orders
            .iter()
            .filter(|order| makable(order.to_string(), &pattern))
            .inspect(|order| println!("✅ {order}"))
            .count() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
