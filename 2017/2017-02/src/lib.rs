pub mod part1 {
    pub fn solve(file: String) -> u32 {
        file.lines()
            .map(|line| {
                let list = line
                    .split_whitespace()
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let min = *list.iter().min().unwrap();
                let max = *list.iter().max().unwrap();
                max - min
            })
            .sum()
    }
}

pub mod part2 {
    use itertools::Itertools;

    pub fn solve(file: String) -> u32 {
        file.lines()
            .map(|line| {
                let list = line
                    .split_whitespace()
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                list.into_iter()
                    .permutations(2)
                    .map(|pair| {
                        let a = pair[0];
                        let b = pair[1];

                        if a.is_multiple_of(b) { a / b } else { 0 }
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}
