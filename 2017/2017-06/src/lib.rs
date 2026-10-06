pub fn parse(file: String) -> Vec<u32> {
    file.split_whitespace()
        .map(|str| str.parse::<u32>().unwrap())
        .collect()
}

pub mod part1 {
    use std::collections::HashSet;

    use crate::parse;

    pub fn solve(file: String) -> usize {
        let mut list = parse(file);
        let len = list.len();
        let mut history = HashSet::new();

        history.insert(list.clone());
        loop {
            let (index, max) = list
                .iter_mut()
                .enumerate()
                .max_by_key(|(i, v)| ((**v as i32) * 1000) - (*i as i32))
                .unwrap();
            let mut temp = *max;
            *max = 0;
            let mut index = (index + 1) % len;
            while temp > 0 {
                let slot = list.get_mut(index).unwrap();
                *slot += 1;
                temp -= 1;
                index = (index + 1) % len;
            }
            if !history.insert(list.clone()) {
                break;
            }
        }
        history.len()
    }
}

pub mod part2 {
    use crate::parse;

    pub fn solve(file: String) -> usize {
        let mut list = parse(file);
        let len = list.len();
        let mut history = Vec::new();

        history.push(list.clone());
        loop {
            let (index, max) = list
                .iter_mut()
                .enumerate()
                .max_by_key(|(i, v)| ((**v as i32) * 1000) - (*i as i32))
                .unwrap();
            let mut temp = *max;
            *max = 0;
            let mut index = (index + 1) % len;
            while temp > 0 {
                let slot = list.get_mut(index).unwrap();
                *slot += 1;
                temp -= 1;
                index = (index + 1) % len;
            }
            if history.contains(&list) {
                history.push(list.clone());
                break;
            }
            history.push(list.clone());
        }
        let last = history.last().unwrap();
        history.len() - history.iter().position(|list| list == last).unwrap() - 1
    }
}
