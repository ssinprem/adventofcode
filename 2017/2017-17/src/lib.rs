pub mod part1 {
    pub fn solve(file: String) -> usize {
        let steps = file.parse::<usize>().expect("cannot parse number");
        let mut list = vec![0];
        let mut current = 0;

        for i in 1..=2017 {
            current = (current + steps) % list.len() + 1;
            list.insert(current, i);
        }

        let idx = list.iter().position(|i| i == &2017).unwrap();
        list[(idx + 1) % list.len()]
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
