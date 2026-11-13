pub fn fuel(mass: usize) -> usize {
    (mass / 3) - 2
}

pub mod part1 {
    use crate::fuel;

    pub fn solve(file: String) -> usize {
        file.lines()
            .map(|line| {
                let mass = line.parse::<usize>().unwrap();
                fuel(mass)
            })
            .sum()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
