pub mod part1 {
    use std::collections::VecDeque;

    pub fn solve(file: String) -> Option<String> {
        let mut list = VecDeque::new();
        list.push_back("".to_string());

        while let Some(path) = list.pop_front() {
            let mut pos = (0, 0);
            for step in path.chars() {
                match step {
                    'U' => pos = (pos.0, pos.1 - 1),
                    'D' => pos = (pos.0, pos.1 + 1),
                    'L' => pos = (pos.0 - 1, pos.1),
                    'R' => pos = (pos.0 + 1, pos.1),
                    _ => {}
                }
            }
            if pos == (3, 3) {
                return Some(path.to_string());
            }

            let hash = format!("{:x}", md5::compute(file.to_string() + path.as_str()));
            let (hash, _) = hash.split_at(4);
            let doors = hash
                .chars()
                .map(|char| match char {
                    '0'..='a' => false,
                    'b'..='f' => true,
                    _ => unreachable!(),
                })
                .collect::<Vec<bool>>();
            doors
                .iter()
                .zip(['U', 'D', 'L', 'R'])
                .for_each(|(open, di)| {
                    let dif = match di {
                        'U' => (0, -1),
                        'D' => (0, 1),
                        'L' => (-1, 0),
                        'R' => (1, 0),
                        _ => unreachable!(),
                    };
                    if *open {
                        let next = (pos.0 + dif.0, pos.1 + dif.1);
                        if next.0 >= 0 && next.0 < 4 && next.1 >= 0 && next.1 < 4 {
                            list.push_back(format!("{}{}", path, di));
                        }
                    }
                });
        }

        None
    }
}

pub mod part2 {
    use std::collections::{HashMap, VecDeque};

    pub fn solve(file: String) -> Option<usize> {
        let mut list = VecDeque::new();
        let mut done_list = HashMap::new();
        list.push_back("".to_string());

        while let Some(path) = list.pop_front() {
            let mut pos = (0, 0);
            for step in path.chars() {
                match step {
                    'U' => pos = (pos.0, pos.1 - 1),
                    'D' => pos = (pos.0, pos.1 + 1),
                    'L' => pos = (pos.0 - 1, pos.1),
                    'R' => pos = (pos.0 + 1, pos.1),
                    _ => {}
                }
            }
            if pos == (3, 3) {
                done_list.insert(path.to_string(), path.len());
                continue;
            }

            let hash = format!("{:x}", md5::compute(file.to_string() + path.as_str()));
            let (hash, _) = hash.split_at(4);
            let doors = hash
                .chars()
                .map(|char| match char {
                    '0'..='a' => false,
                    'b'..='f' => true,
                    _ => unreachable!(),
                })
                .collect::<Vec<bool>>();
            doors
                .iter()
                .zip(['U', 'D', 'L', 'R'])
                .for_each(|(open, di)| {
                    let dif = match di {
                        'U' => (0, -1),
                        'D' => (0, 1),
                        'L' => (-1, 0),
                        'R' => (1, 0),
                        _ => unreachable!(),
                    };
                    if *open {
                        let next = (pos.0 + dif.0, pos.1 + dif.1);
                        if next.0 >= 0 && next.0 < 4 && next.1 >= 0 && next.1 < 4 {
                            list.push_back(format!("{}{}", path, di));
                        }
                    }
                });
        }

        done_list.values().copied().max()
    }
}
