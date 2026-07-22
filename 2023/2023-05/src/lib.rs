use regex::Regex;

#[allow(clippy::type_complexity)]
pub fn parse(file: String) -> (Vec<u64>,Vec<(String, Vec<Vec<u64>>)>) {

    let seeds = file.lines().find(|line| line.starts_with("seeds:")).unwrap()
    .strip_prefix("seeds: ").unwrap()
    .split_whitespace().map(|str| str.parse().unwrap()).collect();
    let regex = Regex::new(r"([a-z\-]+) map:\n([0-9 \n]+)").unwrap();
    // let mut maps = HashMap::<String, Vec<[u64; 3]>>::new();

    let maps = regex.captures_iter(&file).map(|caps| {
        let name = caps.get(1).unwrap().as_str().to_string();
        let lists = caps.get(2).unwrap().as_str().lines().filter_map(|line| {
            if !line.is_empty() {
                Some(
                    line.split_whitespace()
                    .map(|str| str.parse().unwrap()).collect()
                )
            } else {
                None
            }
        }).collect();
        (name, lists)
    }).collect();
    (seeds, maps)
}

pub mod part1 {
    use std::println;

use super::*;

    pub fn solve(file: String) -> u64 {
        let (seeds,type_maps) = parse(file);
        println!("{seeds:?}");
        const MAX: u64 = u64::MAX;
        let mut slots: Vec<_> = seeds.iter().map(|seed| {
            [*seed, MAX, MAX, MAX, MAX, MAX, MAX, MAX]
        }).collect();

        for (id, (_str,maps)) in type_maps.iter().enumerate(){
            for slot in slots.iter_mut() {
                let last = slot[id];
                let mut found = false;
                for map in maps {
                    let &dst = map.first().unwrap();
                    let &src = map.get(1).unwrap();
                    let &len = map.get(2).unwrap();
                    
                    if (src..(src+len)).contains(&last) {
                        let i = last-src;
                        slot[id+1] = dst+i;
                        found = true;
                        break;
                    }
                }
                if !found {
                    slot[id+1] = last;
                }
            }
        }
        slots.iter().map(|slot| slot[7]).min().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
