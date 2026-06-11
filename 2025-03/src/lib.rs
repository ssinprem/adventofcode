pub mod part1 {
    pub fn solve(file: String) -> u64 {
        file.lines().map(|line| line.to_string())
        .map(|line| {
            let mut max: u64 = 0;
            (0..line.len()).for_each(|i| {
                (i+1..line.len()).for_each(|j| {
                    let num = line.chars().nth(i).unwrap().to_digit(10).unwrap()*10 +
                              line.chars().nth(j).unwrap().to_digit(10).unwrap();
                    max = max.max(num as u64);
                });
            });
            max
        })
        .inspect(|&n| { println!("{n}");})
        .sum()
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        file.lines().map(|line| line.to_string())
        .map(|line| {
            0
        })
        .sum()
    }
}