
pub fn parse(file: String) -> (Vec<(String, String)>, String) {
    let mut list = Vec::new();
    let (str_map, init) = file.split_once("\n\n").unwrap();

    str_map.lines().filter(|line| !line.is_empty())
    .for_each(|line| {
        if let Some((key, value)) = line.split_once(" => ") {
            list.push((key.to_string(), value.to_string()));
        }
    });

    (list, init.to_string())
}

pub mod part1 {
    use std::collections::HashSet;
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (map, init) = parse(file);
        let mut list = HashSet::new();
        
        for (key, replace) in map {
            let template = init.replace(key.as_str(), "#");
            let count = template.chars().filter(|c| *c=='#').count();
            for i in 0..count {
                let mut new_string = template.to_string();
                for j in 0..count {
                    if i==j {
                        new_string = new_string.replacen("#", &replace, 1);
                    } else {
                        new_string = new_string.replacen("#", &key, 1);
                    }
                }
                list.insert(new_string);
            }
        }
        list.len() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
