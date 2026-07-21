use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Logic {
    Value(bool),
    AND(String, String),
    OR(String, String),
    XOR(String, String),
}

pub fn get_value(logic: &Logic) -> Option<bool> {
    if let Logic::Value(value) = logic {
        Some(*value)
    } else {
        None
    }
}


pub mod part1 {
    use super::*;
    
    pub fn parse(file: String) -> HashMap<String, Logic> {
        let regex = Regex::new(
            r"(?:([a-z0-9]+): ([0-1]))|(?:([a-z0-9]+) (OR|AND|XOR) ([a-z0-9]+) -> ([a-z0-9]+))",
        )
        .unwrap();
        let caps = regex.captures_iter(&file);
    
        caps.filter_map(|cap| {
            if let Some(var) = cap.get(1)
                && let Some(val) = cap.get(2)
            {
                let logic = val.as_str() != "0";
                Some((var.as_str().to_string(), Logic::Value(logic)))
            } else if let Some(op1) = cap.get(3)
                && let Some(op) = cap.get(4)
                && let Some(op2) = cap.get(5)
                && let Some(target) = cap.get(6)
            {
                let op1 = op1.as_str().to_string();
                let op2 = op2.as_str().to_string();
                let op = op.as_str();
                let target = target.as_str().to_string();

                match op {
                    "OR" => Some((target, Logic::OR(op1, op2))),
                    "AND" => Some((target, Logic::AND(op1, op2))),
                    "XOR" => Some((target, Logic::XOR(op1, op2))),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
    }
    
    pub fn process(map: &mut HashMap<String, Logic>) {
        loop {
            let mut changes = Vec::new();
            for (name, logic) in map.iter() {
                if get_value(logic).is_none() {
                    match logic {
                        Logic::AND(op1, op2) => {
                            if let (Some(v1), Some(v2)) = (
                                map.get(op1).and_then(get_value),
                                map.get(op2).and_then(get_value),
                            ) {
                                changes.push((name.clone(), Logic::Value(v1 && v2)));
                            }
                        }
                        Logic::OR(op1, op2) => {
                            if let (Some(v1), Some(v2)) = (
                                map.get(op1).and_then(get_value),
                                map.get(op2).and_then(get_value),
                            ) {
                                changes.push((name.clone(), Logic::Value(v1 || v2)));
                            }
                        }
                        Logic::XOR(op1, op2) => {
                            if let (Some(v1), Some(v2)) = (
                                map.get(op1).and_then(get_value),
                                map.get(op2).and_then(get_value),
                            ) {
                                changes.push((name.clone(), Logic::Value(v1 != v2)));
                            }
                        }
                        Logic::Value(_) => unreachable!(),
                    }
                }
            }
    
            if changes.is_empty() {
                break;
            }
    
            for (name, new_logic) in changes {
                map.insert(name, new_logic);
            }
        }
    }
    
    pub fn solve(file: String) -> u64 {
        let mut map = parse(file);

        process(&mut map);

        map.iter()
            .filter_map(|(name, logic)| {
                if name.starts_with("z") {
                    let num = name.strip_prefix("z").unwrap().parse::<u16>().unwrap();
                    if let Logic::Value(val) = logic {
                        if *val { Some(1 << num) } else { Some(0) }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum::<u64>()
    }
}

pub mod part2 {

use super::*;

    pub fn parse(file: String) -> Vec<(String, Logic)> {
        let regex = Regex::new(
            r"(?:([a-z0-9]+): ([0-1]))|(?:([a-z0-9]+) (OR|AND|XOR) ([a-z0-9]+) -> ([a-z0-9]+))",
        )
        .unwrap();
        let caps = regex.captures_iter(&file);
    
        caps.filter_map(|cap| {
            if let Some(var) = cap.get(1)
                && let Some(val) = cap.get(2)
            {
                let logic = val.as_str() != "0";
                Some((var.as_str().to_string(), Logic::Value(logic)))
            } else if let Some(op1) = cap.get(3)
                && let Some(op) = cap.get(4)
                && let Some(op2) = cap.get(5)
                && let Some(target) = cap.get(6)
            {
                let op1 = op1.as_str().to_string();
                let op2 = op2.as_str().to_string();
                let op = op.as_str();
                let mut target = target.as_str().to_string();
                target = match target.as_str() {
                    "z12" => "vdc",
                    "vdc" => "z12",
                    "z21" => "nhn",
                    "nhn" => "z21",
                    "tvb" => "khg",
                    "khg" => "tvb",
                    "z33" => "gst",
                    "gst" => "z33",
                    n => n
                }.to_string();
    
                match op {
                    "OR" => Some((target, Logic::OR(op1, op2))),
                    "AND" => Some((target, Logic::AND(op1, op2))),
                    "XOR" => Some((target, Logic::XOR(op1, op2))),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
    }

    pub fn find_deep(map: &Vec<(String, Logic)>, name: String) -> String {
        let logic = &map.iter().find(|(nname,_logic)| nname.to_string() == name).unwrap().1;
        match logic {
            Logic::AND(op1, op2) => {
                let mut vec = vec![
                    find_deep(map, op1.to_string()),
                    find_deep(map, op2.to_string())
                ];
                vec.sort();
                "(".to_string() + &vec[0] + "&" + &vec[1] + ")"
            }
            Logic::OR(op1, op2) => {
                let mut vec = vec![
                    find_deep(map, op1.to_string()),
                    find_deep(map, op2.to_string())
                ];
                vec.sort();
                "(".to_string() + &vec[0] + "|" + &vec[1] + ")"
            }
            Logic::XOR(op1, op2) => {
                let mut vec = vec![
                    find_deep(map, op1.to_string()),
                    find_deep(map, op2.to_string())
                ];
                vec.sort();
                "(".to_string() + &vec[0] + "^" + &vec[1] + ")"
            }
            Logic::Value(_val) => {
                name.to_string()
            }
        }
    }

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        let string_map: Vec<(String,String,Vec<_>)> = map.iter().clone()
            .map(|(name,_logic)| {
                let string = find_deep(&map, name.to_string());
                let regex = Regex::new(r"\([x|y](\d\d)(&|\^)[x|y](\d\d)\)").unwrap();
                let set = regex.captures_iter(&string.clone())
                .map(|cap| {
                    (
                        cap.get(1).unwrap().as_str().to_string(),
                        cap.get(2).unwrap().as_str().to_string()
                    )
                }).collect::<Vec<_>>();
                let mut vec = set.iter()
                    .map(|(s1,s2)| (s1.to_string(),s2.to_string()))
                    .collect::<Vec<_>>();
                vec.sort_by_key(|(s1,s2)| {
                    (s1.to_string(), std::cmp::Reverse(s2.to_string()))
                });
                let vec = vec.iter().map(|(s1,s2)| {
                    s1.to_string() + s2.as_str() 
                }).collect();
                (
                    name.to_string(),
                    string,
                    vec
                )
            }).collect::<Vec<(String,String,_)>>();
        let sort_map: Vec<_> = string_map.iter().clone().collect();
        // sort_map.sort_by_key(|(name,_)| name.to_string());
        sort_map.iter().for_each(|(name, string, vec)| {
            if name.starts_with("x") || name.starts_with("y") {
                return;
            }
            if name.starts_with("z") {
                let mut found = true;
                println!();
                print!("{name} => ");
                
                // println!("{vec:?}");
                let num = name.strip_prefix("z").unwrap().parse::<u64>().unwrap();
                for n in 0..=num {
                    if n>0 {
                        let carry = format!("(x{:02}&y{:02})", n-1, n-1);
                        if !string.contains(&carry)
                        {
                            found = false;
                            println!();
                            println!("❌ not found 'carry' {carry}");
                        }
                    }
                    if num == 0 || n == 0 || (num == 45 && n == 45)
                    {
                        // z45 is special and doesn't have a half adder part
                    } else {
                        let mut output = format!("(x{:02}^y{:02})", n, n);
                        if n==num {
                            output = "^".to_string() + output.as_str();
                        } else {
                            output = "&".to_string() + output.as_str();
                        }
                        // print!("find {output}");
                        if !string.contains(&output)
                        {
                            found = false;
                            println!();
                            println!("❌ not found 'output' {output}");
                        }
                    }
                }
                if found {
                    println!("✅");
                }
                println!("{string}");
            } else {
                print!("{name} => ");
                // println!("{vec:?}");
                if vec.len() == 1 {
                    let regex = Regex::new(r"[x|y](\d\d)(&|\^)[x|y]\d\d").unwrap();
                    let cap = regex.captures(string).unwrap();
                    let op = cap.get(2).unwrap().as_str();
                    let num = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    if op == "^" {
                        println!("⚠️ half adder {num:02}");
                    } else if op == "&" {
                        println!("⚠️ half carry {num:02}");
                    }
                } else {
                    let mut n = 1;
                    let mut result = String::new();
                    loop {
                        let mut carry = format!("(x{:02}&y{:02})", n-1, n-1);
                        if n != 1 {
                            carry = "|".to_string() + carry.as_str();
                        }
                        if string.contains(&carry) {
                            result = format!("⚠️ full carry {:02}", n-1);
                        } else {
                            break;
                        }

                        let adder = format!("&(x{:02}^y{:02})", n, n);
                        let output = format!("^(x{:02}^y{:02})", n, n);
                        if string.contains(&adder) {
                            result = format!("⚠️ carry {:02}", n);
                        } else if string.contains(&output) {
                            result = format!("❓ output {:02}", n);
                        } else {
                            break;
                        }
                        n += 1;
                    }
                    println!("{result}");
                }
                println!("{string}");
            }
        });

        0
    }
}
