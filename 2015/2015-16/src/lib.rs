use regex::Regex;
use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<u32, HashMap<String, u32>> {
    let regex = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
    file.lines()
        .filter_map(|line| {
            if let Some(caps) = regex.captures(line)
                && let Some(no) = caps.get(1)
                && let Ok(no) = no.as_str().parse::<u32>()
                && let Some(name1) = caps.get(2)
                && let Some(amt1) = caps.get(3)
                && let Ok(amt1) = amt1.as_str().parse::<u32>()
                && let Some(name2) = caps.get(4)
                && let Some(amt2) = caps.get(5)
                && let Ok(amt2) = amt2.as_str().parse::<u32>()
                && let Some(name3) = caps.get(6)
                && let Some(amt3) = caps.get(7)
                && let Ok(amt3) = amt3.as_str().parse::<u32>()
            {
                let mut hm = HashMap::new();
                hm.insert(name1.as_str().to_string(), amt1);
                hm.insert(name2.as_str().to_string(), amt2);
                hm.insert(name3.as_str().to_string(), amt3);
                Some((no, hm))
            } else {
                None
            }
        })
        .collect()
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let mut invident = HashMap::new();
        invident.insert("children", 3);
        invident.insert("cats", 7);
        invident.insert("samoyeds", 2);
        invident.insert("pomeranians", 3);
        invident.insert("akitas", 0);
        invident.insert("vizslas", 0);
        invident.insert("goldfish", 5);
        invident.insert("trees", 3);
        invident.insert("cars", 2);
        invident.insert("perfumes", 1);

        let aunts = parse(file);

        let aunt = aunts
            .into_iter()
            .find(|(_no, hm)| {
                hm.iter()
                    .all(|(k, v)| invident.get(k.as_str()).unwrap() == v)
            })
            .expect("cannot find");
        aunt.0 as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
