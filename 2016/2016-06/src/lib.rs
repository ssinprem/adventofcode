pub mod part1 {
    use std::collections::HashMap;

    pub fn solve(file: String) -> String {
        let n = file.lines().next().unwrap().len();
        let mut history = vec![HashMap::<char, usize>::new(); n];

        file.lines()
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                line.chars().enumerate().for_each(|(i, char)| {
                    history[i]
                        .entry(char)
                        .and_modify(|num| *num += 1)
                        .or_insert(1);
                });
            });

        history
            .iter()
            .map(|hm| {
                let (char, _) = hm.iter().max_by_key(|(_, num)| *num).unwrap();
                *char
            })
            .collect()
    }
}

pub mod part2 {
    use std::collections::HashMap;

    pub fn solve(file: String) -> String {
        let n = file.lines().next().unwrap().len();
        let mut history = vec![HashMap::<char, usize>::new(); n];

        file.lines()
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                line.chars().enumerate().for_each(|(i, char)| {
                    history[i]
                        .entry(char)
                        .and_modify(|num| *num += 1)
                        .or_insert(1);
                });
            });

        history
            .iter()
            .map(|hm| {
                let (char, _) = hm.iter().min_by_key(|(_, num)| *num).unwrap();
                *char
            })
            .collect()
    }
}
