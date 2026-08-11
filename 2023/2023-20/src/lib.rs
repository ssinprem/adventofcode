use regex::*;
use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Flipflop,
    Conjuction,
    Broadcaster,
    Error,
}

pub fn parse(file: String) -> HashMap<String, (Type, Vec<String>, Vec<String>)> {
    let regex = Regex::new(r"([%&]*)([a-z]+) -> ([a-z, ]+)").unwrap();
    let hm: HashMap<String, _> = file
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            if let Some(matchs) = regex.captures(line) {
                println!("{matchs:?}");
                let typ = if let Some(t) = matchs.get(1)
                    && !t.is_empty()
                {
                    if t.as_str() == "&" {
                        Type::Conjuction
                    } else if t.as_str() == "%" {
                        Type::Flipflop
                    } else {
                        Type::Error
                    }
                } else {
                    Type::Broadcaster
                };

                let name = matchs.get(2).unwrap().as_str().to_string();
                let str_ports = matchs.get(3).unwrap().as_str().to_string();

                Some((
                    name,
                    (
                        typ,
                        str_ports
                            .split(",")
                            .map(|str| str.trim().to_string())
                            .collect::<Vec<String>>(),
                    ),
                ))
            } else {
                None
            }
        })
        .collect();
    let org = hm.clone();
    hm.iter()
        .map(|(name, (typ, dst))| {
            let mut src = Vec::new();

            if *typ == Type::Conjuction {
                src = org
                    .iter()
                    .filter_map(|(src_name, (_typ, target))| {
                        if target.contains(name) {
                            Some(src_name.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();
            }
            (name.to_string(), (typ.clone(), dst.clone(), src))
        })
        .collect()
}

pub mod part1 {
    use super::*;

    pub fn _print_state(states :&HashMap<&String, bool>) -> String {
        let mut strings = "".to_string();
        states.iter().for_each(|(node, state)| {
            strings += format!("{node}:{state:6} ").as_str();
        });
        strings
    }

    pub fn solve(file: String) -> u64 {
        let mut low_cnt = 0;
        let mut high_cnt = 0;
        let hm = parse(file);

        println!("{hm:?}");
        let mut states: HashMap<_, _> = hm
            .iter()
            .filter_map(|(name, (typ, _next, _src))| {
                match typ {
                    Type::Flipflop => Some((name, false)),
                    Type::Conjuction => Some((name, true)),
                    _ => None
                }
                
                // Type::Broadcaster => {
                //     None
                // },
                //     Type::Error => false,
                // (
                //     name,
                
                // )
            })
            .collect();

        let mut first_states = states.clone();
        let mut sender = Vec::new();
        let mut round = 0;
        loop {
            println!("{round} {high_cnt} {low_cnt}");
            round += 1;
            low_cnt += 1;
            println!(" button           false   broadcaster");
            sender.push(("broadcaster".to_string(), false));
            while !sender.is_empty()
                && let (des, sig) = sender.remove(0)
            {
                if let Some((typ, nodes, srcs)) = hm.get(&des.to_string()) {
                    if *typ == Type::Broadcaster {
                        for node in nodes {
                            if sig {
                                high_cnt += 1;
                            } else {
                                low_cnt += 1;
                            }
                            println!(" {des:15}  {sig:6}  {node:15} {}",_print_state(&states));
                            sender.push((node.to_string(), sig));
                        }
                    } else if *typ == Type::Flipflop {
                        if !sig {
                            let next_state = {
                                let state = states.get_mut(&des).unwrap();
                                *state = !*state;
                                *state
                            };
                            for node in nodes {
                                if next_state {
                                    high_cnt += 1;
                                } else {
                                    low_cnt += 1;
                                }
                                sender.push((node.to_string(), next_state));

                                println!(" {des:15}  {next_state:6}  {node:15} {}", _print_state(&states));
                            }
                        }
                    } else if *typ == Type::Conjuction {
                        let og_states = states.clone();
                        let old_state = states.get_mut(&des).unwrap();
    
                        let src_state = !og_states
                            .iter()
                            .filter(|(name, _state)| srcs.contains(name))
                            .all(|(_n, state)| *state);
    
                        
                        *old_state = src_state;
                        if src_state {
                            high_cnt += 1;
                        } else {
                            low_cnt += 1;
                        }
                        for node in nodes {
                            println!(" {des:15}  {src_state:6}  {node:15} {}",_print_state(&states));
                            sender.push((node.to_string(), src_state))
                        }
                        
                    }
                }
            }
            println!("round {round} {}", _print_state(&states));
            if round >= 1000 {
                break;
            } 
            else if hm.iter().filter_map(|(name,(typ, _,_))| {
                    if *typ == Type::Flipflop {
                        Some(name)
                    } else {
                        None
                    }
                }).all(|name| !*states.get(name).unwrap())
            {
                break;
            }
        }
        println!("{round} {high_cnt} {low_cnt}");
        let mul = 1000/round;
        high_cnt * low_cnt *mul*mul
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
