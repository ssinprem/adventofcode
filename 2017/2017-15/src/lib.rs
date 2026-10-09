pub fn parse(file: String) -> (u32, u32) {
    let vec = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect::<Vec<u32>>();

    (vec[0], vec[1])
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (gen_a, gen_b) = parse(file);
        const FACTOR_A: u64 = 16807;
        const FACTOR_B: u64 = 48271;
        const MAX: u64 = 2147483647;
        const TARGET_BIT: u32 = 0b1111_1111_1111_1111;
        let mut val_a = gen_a;
        let mut val_b = gen_b;

        let mut match_pair = 0;
        for _ in 0..40_000_000 {
            val_a = ((val_a as u64 * FACTOR_A) % MAX) as u32;
            val_b = ((val_b as u64 * FACTOR_B) % MAX) as u32;

            if val_a & TARGET_BIT == val_b & TARGET_BIT {
                match_pair += 1;
            }
        }
        match_pair
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
