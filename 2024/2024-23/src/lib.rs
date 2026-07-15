use std::collections::HashSet;

pub fn parse(file: String) -> (HashSet<String>, Vec<HashSet<String>>) {
    let mut list: HashSet<String> = HashSet::new();
    let mut pairs: Vec<HashSet<String>> = Vec::new();
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if let Some((mc1, mc2)) = line.split_once("-") {
                list.insert(mc1.to_string());
                list.insert(mc2.to_string());
                let mut pair = HashSet::<String>::new();
                pair.insert(mc1.to_string());
                pair.insert(mc2.to_string());
                pairs.push(pair);
            }
        });
    (list, pairs)
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (lists, pairs) = parse(file);

        let tmc: Vec<_> = lists.iter().filter(|mc| mc.starts_with("t")).collect();

        let mut groups: HashSet<Vec<String>> = HashSet::new();
        for mc1 in tmc {
            let second_nexts: Vec<_> = pairs
                .iter()
                .filter_map(|set| {
                    if set.iter().any(|mc| mc == mc1) {
                        Some(set.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for next2 in second_nexts {
                let mc2 = next2.iter().find(|mc| *mc != mc1).unwrap();
                let third_nexts = pairs.iter().filter_map(|set| {
                    if set.iter().any(|mc| mc == mc2) && set.iter().all(|mc| mc != mc1) {
                        Some(set.clone())
                    } else {
                        None
                    }
                });

                for next3 in third_nexts {
                    let mc3 = next3.iter().find(|mc| *mc != mc2).unwrap();
                    if pairs
                        .iter()
                        .any(|set| set.contains(mc1) && set.contains(mc3))
                    {
                        let mut group = vec![mc1.to_string(), mc2.to_string(), mc3.to_string()];
                        group.sort();
                        groups.insert(group);
                    }
                }
            }
        }
        groups.len() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
