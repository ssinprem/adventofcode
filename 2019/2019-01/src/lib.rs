pub fn fuel(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
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
use crate::fuel;

    pub fn solve(file: String) -> usize {
        file.lines()
            .map(|line| {
                let mass = line.parse::<usize>().unwrap();
                let mut tank = 0;
                let mut fuels = fuel(mass);
                tank += fuels;
                while let fuel_for_fuel = fuel(fuels)
                && fuel_for_fuel > 0
                {
                    tank += fuel_for_fuel;
                    fuels = fuel_for_fuel;
                }
                tank
            })
            .sum()
    }
}
