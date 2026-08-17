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
    let regex = Regex::new(r"([%&]*)([a-z0-9]+) -> ([a-z0-9, ]+)").unwrap();
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

pub fn init_state(
    hm: &HashMap<String, (Type, Vec<String>, Vec<String>)>,
) -> (
    HashMap<String, bool>,
    HashMap<String, HashMap<String, bool>>,
) {
    (
        hm.iter()
            .filter_map(|(name, (typ, _next, _src))| {
                if *typ == Type::Flipflop {
                    Some((name.clone(), false))
                } else {
                    None
                }
            })
            .collect(),
        hm.iter()
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
            .collect(),
    )
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

        let (mut ff_states, mut conj_states) = init_state(&hm);

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
    use super::*;

    fn gcd(mut a: u64,mut b: u64) -> u64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    fn lcm(a: u64, b: u64) -> u64 {
        (a*b) / gcd(a,b)
    }

    fn lcms(list : &[u64]) -> u64 {
        list.iter().fold(list[0],|acc, n| lcm(acc,*n))
    }

    pub fn solve(file: String) -> u64 {
        let hm = parse(file);

        let (mut ff_states, mut conj_states) = init_state(&hm);

        let mut sender = Vec::new();
        let mut round = 0;

        let mut src_rx : Vec<_> = hm.iter().filter_map(|(name,(typ,des,_src))| {
            if des.contains(&"rx".to_string()) {
                Some((name.to_string(), typ.clone(), false))
            } else {
                None
            }
        }).collect();
        println!("{src_rx:?}");
        
        while src_rx.iter().any(|(_name,type_, state)| type_ == &Type::Conjuction && state == &false) {
            if let Some(pos) = src_rx.iter().position(|(_name,type_, state)| {
                type_ == &Type::Conjuction && state == &false
            }) {
                let (dst_name, _, bool) = src_rx.remove(pos);
                let (_t, _dst, src)= hm.get(&dst_name).unwrap();
                for s in src {
                    let t = (*hm.get(s).unwrap()).0.clone();
                    src_rx.push((s.to_string(), t, !bool));
                }
            }
        }
        println!("{src_rx:?}");
        let mut src_rx : HashMap<_,_> = src_rx.iter().map(| (name, _type, bool )| {
            (name.to_string(), (*bool, 0))
        }).collect();
        loop {
            // println!("{round}");
            sender.push(("broadcaster".to_string(), "button".to_string(), false));
            while !sender.is_empty()
                && let (des, src, sig) = sender.remove(0)
            {
                // println!("{src:10} {sig:10} {des:10}");
                if des == "rx" && !sig {
                    return round;
                }
                
                if let Some((typ, nodes, _srcs)) = hm.get(&des) {
                    if *typ == Type::Broadcaster {
                        for node in nodes {
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
                                sender.push((node.to_string(), des.to_string(), next_state));
                            }
                        }
                    } else if *typ == Type::Conjuction {
                        let mem = conj_states.get_mut(&des).unwrap();
                        mem.insert(src, sig);
                        let next_sig = !mem.values().all(|v| *v);
                        for node in nodes {
                            sender.push((node.to_string(), des.to_string(), next_sig));
                        }
                    }
                }
            }
            round += 1;


            if src_rx.clone().iter().all(|(_name, (_, n))| *n != 0) {
                break;
            }
            // println!("{conj_states:?} {ff_states:?}");
            println!("{src_rx:?}");
            for (pre_rx, (expect, num)) in src_rx.iter_mut() {
                if *num == 0 {
                    let val = 
                    if let Some(mem) = conj_states.get(pre_rx) {
                        !mem.values().all(|v| *v)
                    } else if let Some(state) = ff_states.get(pre_rx) {
                            *state
                        } else {
                            unreachable!()
                        };
                    println!("{pre_rx} {expect} {val}");
                    if *expect == val {
                        *num = round;
                    }
                }
            }
        }
        println!("{src_rx:?}");
        lcms(src_rx.iter().map(|(_name,(_,n))| *n).collect::<Vec<u64>>().as_slice())
    }
}
