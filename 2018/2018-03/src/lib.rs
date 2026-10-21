use regex::Regex;

pub fn parse(file: String) -> Vec<(usize, usize, usize, usize, usize)> {
    let regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    file.lines()
        .map(|line| {
            let matched = regex.captures(line).unwrap();
            let id = matched.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let x = matched.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let y = matched.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let w = matched.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let h = matched.get(5).unwrap().as_str().parse::<usize>().unwrap();
            (id, x, y, w, h)
        })
        .collect()
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::parse;

    pub fn solve(file: String) -> usize {
        let boxes = parse(file);
        let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        for (id, x, y, w, h) in boxes {
            for dy in 0..h {
                for dx in 0..w {
                    map.entry((x + dx, y + dy))
                        .and_modify(|mem| {
                            mem.push(id);
                        })
                        .or_insert(vec![id]);
                }
            }
        }

        map.values().filter(|mem| mem.len() > 1).count()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
