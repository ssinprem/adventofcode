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

    pub fn _print_state(states: &HashMap<&String, bool>) -> String {
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

        let mut ff_states: HashMap<String, bool> = hm
            .iter()
            .filter_map(|(name, (typ, _next, _src))| {
                if *typ == Type::Flipflop {
                    Some((name.clone(), false))
                } else {
                    None
                }
            })
            .collect();

        let mut conj_states: HashMap<String, HashMap<String, bool>> = hm
            .iter()
            .filter_map(|(name, (typ, _, src))| {
                if *typ == Type::Conjuction {
                    Some((
                        name.clone(),
                        src.iter().map(|s| (s.clone(), false)).collect(),
                    ))
                } else {
                    None
                }
            })
            .collect();

        let mut sender = Vec::new();
        let mut round = 0;
        while round < 1000 {
            low_cnt += 1;
            sender.push(("broadcaster".to_string(), "button".to_string(), false));
            while !sender.is_empty()
                && let (des, src, sig) = sender.remove(0)
            {
                if let Some((typ, nodes, _srcs)) = hm.get(&des) {
                    if *typ == Type::Broadcaster {
                        for node in nodes {
                            if sig {
                                high_cnt += 1;
                            } else {
                                low_cnt += 1;
                            }
                            sender.push((node.to_string(), des.to_string(), sig));
                        }
                    } else if *typ == Type::Flipflop {
                        if !sig {
                            let next_state = {
                                let state = ff_states.get_mut(&des).unwrap();
                                *state = !*state;
                                *state
                            };
                            for node in nodes {
                                if next_state {
                                    high_cnt += 1;
                                } else {
                                    low_cnt += 1;
                                }
                                sender.push((node.to_string(), des.to_string(), next_state));
                            }
                        }
                    } else if *typ == Type::Conjuction {
                        let mem = conj_states.get_mut(&des).unwrap();
                        mem.insert(src, sig);
                        let next_sig = !mem.values().all(|v| *v);
                        for node in nodes {
                            if next_sig {
                                high_cnt += 1;
                            } else {
                                low_cnt += 1;
                            }
                            sender.push((node.to_string(), des.to_string(), next_sig));
                        }
                    }
                }
            }
            round += 1;
            if 1000 % round == 0 && ff_states.iter().all(|(_name, state)| !*state) {
                break;
            }
        }
        let mul = 1000 / round;
        high_cnt * low_cnt * mul * mul
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
