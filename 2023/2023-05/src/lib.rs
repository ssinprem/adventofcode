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
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (seeds,type_maps) = parse(file);
        let mut slots: Vec<_> = Vec::<[u64; 8]>::new();
        println!("{seeds:?}");
        const MAX: u64 = u64::MAX;
        // seeds to soil initial slots
        for map in type_maps.first().unwrap().1.clone() {
            let dst = map.first().unwrap();
            let src = map.get(1).unwrap();
            let len = map.get(2).unwrap();
            for i in 0..*len {
                
                slots.push([src+i, dst+i, MAX, MAX, MAX, MAX, MAX, MAX]);
            }
        }
        // fill empty slot with default
        let min_soil = slots.iter().min_by_key(|s| s[0]).unwrap()[0];
        for i in 0..min_soil {
            slots.push([i,i,MAX,MAX,MAX,MAX,MAX,MAX]);
        }
        slots.sort_by_key(|s| s[0]);

        for (id, (_str,maps)) in type_maps.iter().enumerate(){
            if id == 0{
                continue;
            }
            for map in maps {
                let dst = map.first().unwrap();
                let src = map.get(1).unwrap();
                let len = map.get(2).unwrap();
                for i in 0..*len {
                    let pos = slots.iter().position(|s| s[id]==src+i).unwrap();
                    let slot = slots.get_mut(pos).unwrap();
                    slot[id+1] = dst+i;
                }
            }
            // fill the non map with latest position
            slots.iter_mut().for_each(|slot| {
                if slot[id+1] == MAX {
                    slot[id+1] = slot[id];
                }
            })
        }
        
        seeds.iter().map(|seed| {
            slots.iter()
                .find(|slot| slot[0]==*seed).unwrap()
                [7]
        }).min().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
