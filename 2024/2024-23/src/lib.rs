use std::collections::{HashMap, HashSet};

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

fn build_adjacency_list(pairs: &[HashSet<String>]) -> HashMap<String, HashSet<String>> {
    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
    for pair in pairs {
        let p: Vec<_> = pair.iter().collect();
        let (mc1, mc2) = (p[0], p[1]);
        adj.entry(mc1.clone()).or_default().insert(mc2.clone());
        adj.entry(mc2.clone()).or_default().insert(mc1.clone());
    }
    adj
}
pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (lists, pairs) = parse(file);
        let adj_list = build_adjacency_list(&pairs);

        let tmc: Vec<_> = lists.iter().filter(|mc| mc.starts_with("t")).collect();

        let mut groups: HashSet<Vec<String>> = HashSet::new();
        for mc1 in tmc {
            let second_nexts: Vec<_> = adj_list.get(mc1).unwrap().iter().collect();
            for mc2 in second_nexts {
                let third_nexts: Vec<_> = adj_list.get(mc2).unwrap()
                    .iter().filter(|mc| mc != &mc1)
                    .collect();
                for mc3 in third_nexts {
                    let loop_back = adj_list.get(mc3).unwrap();
                    if loop_back.contains(mc1)
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
    use super::*;

    pub fn solve(file: String) -> String {
        let (list_set, pairs) = parse(file);
        let adj_list = build_adjacency_list(&pairs);

        let mut list: Vec<String> = list_set.into_iter().collect();
        list.sort();
        let mut groups = Vec::<Vec<String>>::new();

        for mc in list {
            for group in groups.iter_mut() {
                let is_compatible = group.iter().all(|mc2| {
                    // This check is now much faster
                    adj_list
                        .get(&mc)
                        .map_or(false, |neighbors| neighbors.contains(mc2))
                });
                if is_compatible {
                    group.push(mc.clone());
                }
            }
            // create new group with only latest machine
            groups.push(vec![mc.clone()]);
        }

        let large_grp = groups.iter().max_by_key(|group| group.len()).unwrap();
        println!("{large_grp:?}");
        large_grp.join(",")
    }
}
