pub fn parse(file: String) -> (Vec<(String, String)>, String) {
    let mut list = Vec::new();
    let (str_map, init) = file.split_once("\n\n").unwrap();

    str_map
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if let Some((key, value)) = line.split_once(" => ") {
                list.push((key.to_string(), value.to_string()));
            }
        });

    (list, init.to_string())
}

pub mod part1 {
    use crate::parse;
    use std::collections::HashSet;

    pub fn solve(file: String) -> u64 {
        let (map, init) = parse(file);
        let mut list = HashSet::new();

        for (key, replace) in map {
            let template = init.replace(key.as_str(), "#");
            let count = template.chars().filter(|c| *c == '#').count();
            for i in 0..count {
                let mut new_string = template.to_string();
                for j in 0..count {
                    if i == j {
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
    use std::collections::HashSet;

    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (map, target) = parse(file);
        let map = map
            .iter()
            .map(|(k, v)| (v.to_string(), k.to_string()))
            .collect::<Vec<(String, String)>>();

        let mut list = vec![target];
        let mut count = 0;
        loop {
            let mut new_list = HashSet::new();
            while let Some(string) = list.pop() {
                for (key, replace) in map.clone().into_iter() {
                    let template = string.replace(key.as_str(), "#");
                    let cnt = template.chars().filter(|c| *c == '#').count();
                    for i in 0..cnt {
                        let mut new_string = template.to_string();
                        for j in 0..cnt {
                            if i == j {
                                new_string = new_string.replacen("#", &replace, 1);
                            } else {
                                new_string = new_string.replacen("#", &key, 1);
                            }
                        }
                        if new_string == "e" {
                           return count+1;
                        }
                        if new_string.chars().filter(|c| c==&'e').count() > 0 {
                           continue;
                        }
                        new_list.insert(new_string);
                    }
                }
            }
            list = new_list.into_iter().collect::<Vec<String>>();
            println!("{count} {}", list.len());
            count += 1;
            if list.contains(&"e".to_string()) {
                return count;
            }
        }
    }
}
