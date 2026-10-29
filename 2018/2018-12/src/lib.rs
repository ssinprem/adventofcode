use std::{collections::HashMap};

pub fn parse(file: String) -> (HashMap<isize,String>, HashMap<String, String>) {
    let mut lines = file.lines().into_iter();

    let initial = lines.next().unwrap();
    let (_,initial) = initial.split_once(":").unwrap();
    let initial = initial.trim().to_string();
    let mut initial = initial.chars().enumerate()
        .into_iter().map(|(index,char)| {
            (index as isize, format!("{char}"))
        }).collect::<HashMap<isize, String>>();
    
    initial.insert(-4, ".".to_string());
    initial.insert(-3, ".".to_string());
    initial.insert(-2, ".".to_string());
    initial.insert(-1, ".".to_string());

    let map = lines.filter(|line| !line.is_empty())
    .map(|line| {
        let (pat, res) = line.split_once(" => ").unwrap();
        (pat.trim().to_string(), res.trim().to_string())
    }).collect::<HashMap<_,_>>();

    (initial, map)
}

pub fn _display (plants: &HashMap<isize, String>) -> String {
    let mut output = String::new();
    let min = *plants.keys().min().unwrap();
    let max = *plants.keys().max().unwrap();

    for i in min..=max {
        if let Some(val) = plants.get(&i) {
            output += val.as_str()
        } else {
            unreachable!();
        }
    }
    output
}

pub fn evolve (plants: &HashMap<isize, String>, map: &HashMap<String, String>) -> HashMap<isize, String> {
    use rayon::prelude::*;
    
    let min = *plants.keys().min().unwrap();
    let max = *plants.keys().max().unwrap();
    

    (min..=max+1).into_par_iter().map(|i| {
        let pat = (-2..=2)
            .map(|d| plants.get(&(i+d)).unwrap_or(&".".to_string()).chars().next().unwrap())
            .collect::<String>();

        if map.get(&pat) == Some(&"#".to_string()) {
            (i, "#".to_string())
        } else {
            (i, ".".to_string())
        }
    }).collect()
}

pub mod part1 {
    use crate::{_display, evolve, parse};
    pub fn solve(file: String) -> i64 {

        let (init, map) = parse(file);
        let mut plants = init.clone();
        println!("  0: {}", _display(&plants));
        for _gen in 1..=20 {
            plants = evolve(&plants, &map);
            println!("{_gen:3}: {}", _display(&plants));
        }

        plants.into_iter().map(|(index, char)| {
            if char.as_str() == "#" {
                index as i64
            } else {
                0
            }
        }).sum()
    }
}

pub mod part2 {
    use crate::{_display, evolve, parse};
    
    pub fn solve(file: String) -> u64 {

        let (init, map) = parse(file);
        let mut plants = init.clone();
        println!("{0:10}: {}", _display(&plants));
        let mut last_sum = plants.iter().map(|(index, char)| {
                if char.as_str() == "#" {
                    *index as i64
                } else {
                    0
                }
            }).sum::<i64>();
        let mut last_diff = i64::MAX;
        let mut last_gen = 0;
        let mut last_output = _display(&plants);
        for generation in 1.. {
            plants = evolve(&plants, &map);
            let sum = plants.iter().map(|(index, char)| {
                if char.as_str() == "#" {
                    *index as i64
                } else {
                    0
                }
            }).sum::<i64>();
            let output = _display(&plants);
            println!("{generation:10}: {sum:10} {output}");
            let diff = sum - last_sum;
            
            last_diff = diff;
            last_sum = sum;
            if output == ".".to_string() + last_output.as_str() {
                last_gen = generation;
                break;
            }
            last_output = output;
        }
        println!("last gen - {last_gen}  {last_sum} {last_diff}");

        last_sum as u64 + ((50_000_000_000 - last_gen as u64) * last_diff as u64)
    }
}

