pub mod part1 {
    use std::collections::HashSet;

    pub fn solve(file: String) -> u64 {
        let mut history = HashSet::new();
        let mut curr = (0,0);
        history.insert(curr);
        file.chars().for_each(|step| {
            match step {
                '^' => curr = (curr.0, curr.1 - 1),
                'v' => curr = (curr.0, curr.1 + 1),
                '<' => curr = (curr.0 - 1, curr.1),
                '>' => curr = (curr.0 + 1, curr.1),
                _ => {}
            }
            history.insert(curr);
        });
        history.len() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
