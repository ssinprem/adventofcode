use std::collections::HashMap;

pub fn find_dist(file: String) -> HashMap<(isize, isize), usize> {
    use std::collections::{HashMap, VecDeque};
    let mut curr = (0, 0);
    let mut dist: HashMap<(isize, isize), usize> = HashMap::new();
    let mut stack = VecDeque::new();
    dist.insert((0, 0), 0);
    for char in file.chars() {
        if "NEWS".contains(char) {
            let mut next = curr;
            match char {
                'N' => next = (next.0, next.1 - 1),
                'E' => next = (next.0 + 1, next.1),
                'W' => next = (next.0 - 1, next.1),
                'S' => next = (next.0, next.1 + 1),
                _ => unreachable!(),
            };
            let curr_dist = *dist.get(&curr).unwrap();
            let next_dist = *dist.get(&next).unwrap_or(&usize::MAX);
            let next_dist = (curr_dist + 1).min(next_dist);
            dist.insert(next, next_dist);
            curr = next;
        } else if char == '(' {
            stack.push_back(curr);
        } else if char == '|' {
            curr = *stack.iter().last().unwrap();
        } else if char == ')' {
            curr = stack.pop_back().unwrap();
        }
    }
    dist
}

pub mod part1 {
    use crate::find_dist;

    pub fn solve(file: String) -> usize {
        let dist = find_dist(file);
        *dist.values().max().unwrap()
    }
}

pub mod part2 {
    use crate::find_dist;

    pub fn solve(file: String) -> usize {
        let dist = find_dist(file);

        dist.iter().filter(|(_pos, d)| **d >= 1000).count()
    }
}
