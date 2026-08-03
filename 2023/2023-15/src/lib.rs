pub mod part1 {
    fn calc(str: String) -> u64 {
        str.chars()
            .filter(|c| !c.is_whitespace())
            .fold(0_u64, |acc, char| {
                let value: u64 = acc + char as u64;
                (value * 17) % 256
            })
    }

    pub fn solve(file: String) -> u64 {
        file.split(",")
            .inspect(|c| print!("{c} "))
            .map(|str| calc(str.to_string()))
            .inspect(|c| println!("= {c}"))
            .sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
