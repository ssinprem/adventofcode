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
    use rand::seq::SliceRandom;
    
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let mut rng = rand::rng();
        let (map, target) = parse(file);
        let mut map = map
            .iter()
            .map(|(k, v)| (v.to_string(), k.to_string()))
            .collect::<Vec<(String, String)>>();

        let mut count = 0;
        let mut current = target.to_string();

        while current != "e" {
            let mut changed = false;

            for (key, replace) in map.clone().into_iter() {
                if current.contains(&key) {
                    current = current.replacen(&key, &replace, 1);
                    count+=1;
                    changed = true;
                    break;
                }
            }
            
            if ! changed {
                current = target.to_string();
                count = 0;
                map.shuffle(&mut rng);
            }
        }
        count
    }
}
