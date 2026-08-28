pub mod part1 {
    pub fn solve(file: String) -> i64 {
        file.chars().filter_map(|char| {
            match char {
                '(' => Some(1),
                ')' => Some(-1),
                _ => None
            }
        }).sum()
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        let mut curr = 0;
        let position = file.chars().filter(|char| ['(',')'].contains(char))
        .position(| c | {
            curr += match c {
                '(' => 1,
                ')' => -1,
                _ => 0
            };
            curr <= -1
        });

        if position.is_some() {
            position.unwrap() as u64 + 1
        } else {
            0
        }
    }
}
