pub mod part1 {
    use std::collections::{HashMap, VecDeque};

    pub fn solve(file: String) -> usize {
        let mut curr = (0,0);
        let mut dist: HashMap<(isize, isize), usize> = HashMap::new();
        let mut stack = VecDeque::new();
        dist.insert((0,0), 0);
        for char in file.chars() {
            if "NEWS".contains(char) {
                let mut next = curr;
                match char {
                    'N' => {
                        next = (next.0, next.1 - 1)
                    },
                    'E' => {
                        next = (next.0 + 1, next.1)
                    },
                    'W' => {
                        next = (next.0 - 1, next.1)
                    },
                    'S' => {
                        next = (next.0, next.1 + 1)
                    },
                    _ => unreachable!()
                };
                let curr_dist = *dist.get(&curr).unwrap();
                let next_dist = *dist.get(&next).unwrap_or(&usize::MAX);
                let next_dist = (curr_dist + 1).min(next_dist);
                dist.insert(next,next_dist);
                curr = next;
            } else if char == '(' {
                stack.push_back(curr);
            } else if char == '|' {
                curr = *stack.iter().last().unwrap();
            } else if char == ')' {
                curr = stack.pop_back().unwrap();
            }
        }

        *dist.values().max().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
