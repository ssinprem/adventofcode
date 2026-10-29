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

pub mod part1 {
    use std::collections::HashMap;

use crate::{_display, parse};

    pub fn solve(file: String) -> i64 {

        let (init, map) = parse(file);
        let mut plants = init.clone();
        println!("  0: {}", _display(&plants));
        for _gen in 1..=20 {
            let min = *plants.keys().min().unwrap();
            let max = *plants.keys().max().unwrap();
            let mut next = HashMap::new();

            for i in min..=max+1 {
                let pat = (-2..=2)
                    .map(|d| plants.get(&(i+d)).unwrap_or(&".".to_string()).chars().next().unwrap())
                    .collect::<String>();

                if map.get(&pat) == Some(&"#".to_string()) {
                    next.insert(i, "#".to_string());
                } else {
                    next.insert(i, ".".to_string());
                }
            }
            plants = next.clone();
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
    pub fn solve(_file: String) -> u64 {
        0
    }
}

