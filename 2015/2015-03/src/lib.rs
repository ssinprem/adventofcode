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
    use std::collections::HashSet;
    pub fn solve(file: String) -> u64 {
        let mut history = HashSet::new();
        let mut santa = (0,0);
        let mut robot = (0,0);
        history.insert(santa);
        file.chars().enumerate().for_each(|(n,step)| {
            if n %2 == 0 {
                match step {
                    '^' => santa = (santa.0, santa.1 - 1),
                    'v' => santa = (santa.0, santa.1 + 1),
                    '<' => santa = (santa.0 - 1, santa.1),
                    '>' => santa = (santa.0 + 1, santa.1),
                    _ => {}
                }
                history.insert(santa);
            } else {
                match step {
                    '^' => robot = (robot.0, robot.1 - 1),
                    'v' => robot = (robot.0, robot.1 + 1),
                    '<' => robot = (robot.0 - 1, robot.1),
                    '>' => robot = (robot.0 + 1, robot.1),
                    _ => {}
                }
                history.insert(robot);
            }
        });
        history.len() as u64
    }
}
