pub mod part1 {
    pub fn solve(file: String) -> u64 {
        let mut list = file
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut step = 0;
        let mut index = 0;

        while let Some(item) = list.get_mut(index as usize) {
            index = ((index as i32) + *item) as isize;
            *item += 1;
            step += 1;
        }
        step
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
