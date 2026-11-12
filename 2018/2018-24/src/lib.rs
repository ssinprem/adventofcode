#[derive(Debug, PartialEq, Clone)]
pub enum Team {
    Immune,
    Infection,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Radiation,
    Bludgeoning,
    Slashing,
    Fire,
    Cold,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    name: String,
    team: Team,
    initiative: usize,
    units: usize,
    hp: usize,
    atk: usize,
    atype: Type,
    weak: Vec<Type>,
    immune: Vec<Type>,
}

pub fn parse(file: String) -> Vec<Group> {
    use regex::Regex;
    let regex = Regex::new(
        r"(\d+) units each with (\d+) hit points([\(\)a-z, ;]+)with an attack that does (\d+) (\w+) damage at initiative (\d+)"
    ).unwrap();
    let mut group = None;
    let mut groups = vec![];
    let mut a = 1;
    let mut b = 1;
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if line == "Immune System:" {
                group = Some(0);
                return;
            } else if line == "Infection:" {
                group = Some(1);
                return;
            }
            let caps = regex.captures(line).unwrap();
            let units = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let hp = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let mut weak = vec![];
            let mut immune = vec![];
            if let Some(attr) = caps.get(3) {
                let mut with = None;
                attr.as_str().split_whitespace().for_each(|word| {
                    let word = word
                        .chars()
                        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_whitespace())
                        .collect::<String>();
                    let (w, t) = match word.as_str() {
                        "weak" => (Some(0), None),
                        "to" => (None, None),
                        "immune" => (Some(1), None),
                        "radiation" => (None, Some(Type::Radiation)),
                        "bludgeoning" => (None, Some(Type::Bludgeoning)),
                        "slashing" => (None, Some(Type::Slashing)),
                        "fire" => (None, Some(Type::Fire)),
                        "cold" => (None, Some(Type::Cold)),
                        _ => unreachable!(),
                    };
                    if let Some(w) = w {
                        with = Some(w);
                    }
                    if let Some(t) = t {
                        if with == Some(0) {
                            weak.push(t);
                        } else if with == Some(1) {
                            immune.push(t);
                        } else {
                            unreachable!();
                        }
                    }
                });
            }
            let atk = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let atype = match caps.get(5).unwrap().as_str() {
                "radiation" => Type::Radiation,
                "bludgeoning" => Type::Bludgeoning,
                "slashing" => Type::Slashing,
                "fire" => Type::Fire,
                "cold" => Type::Cold,
                _ => unreachable!(),
            };
            let initiative = caps.get(6).unwrap().as_str().parse::<usize>().unwrap();
            if group == Some(0) {
                groups.push(Group {
                    team: Team::Immune,
                    name: format!("A{:04}", a),
                    initiative,
                    units,
                    hp,
                    atk,
                    atype,
                    weak,
                    immune,
                });
                a += 1;
            } else if group == Some(1) {
                groups.push(Group {
                    team: Team::Infection,
                    name: format!("B{:04}", b),
                    initiative,
                    units,
                    hp,
                    atk,
                    atype,
                    weak,
                    immune,
                });
                b += 1;
            } else {
                unreachable!();
            }
        });

    groups.sort_by_key(|g| std::cmp::Reverse(g.initiative));

    groups
}

pub fn battle(groups: &mut Vec<Group>) {
    use std::cmp::Reverse;

    // target selection  , sort from Effective Power, Initiative
    groups.sort_by_key(|g| (Reverse(g.units * g.atk), Reverse(g.initiative)));
    let mut taken = vec![];
    for g in groups.clone() {
        let mut target = vec![];
        let enemys = groups
            .iter()
            .filter(|e| e.team != g.team)
            .collect::<Vec<_>>();
        let atk = g.atk * g.units;

        for enemy in enemys {
            if taken
                .iter()
                .map(|(_init, _a, t)| t)
                .collect::<Vec<_>>()
                .contains(&&enemy.name)
            {
                continue;
            }

            let mut damage = atk;
            let eff_pwr = enemy.units * enemy.atk;
            if enemy.weak.contains(&g.atype) {
                damage *= 2;
            } else if enemy.immune.contains(&g.atype) {
                continue;
            }

            target.push((enemy.name.to_string(), damage, eff_pwr, enemy.initiative));
        }
        target.sort_by_key(|t| (Reverse(t.1), Reverse(t.2), Reverse(t.3)));
        if !target.is_empty() {
            taken.push((g.initiative, g.name, target.first().unwrap().0.to_string()));
        }
    }

    // attacking  sort by Initiative
    taken.sort_by_key(|(init, _, _)| Reverse(*init));
    for (_init, a, d) in taken {
        let atk = groups.iter().find(|g| g.name == a).unwrap();
        let (atk_power, atk_type) = (atk.atk * atk.units, atk.atype.clone());
        let def_id = groups.iter().position(|g| g.name == d).unwrap();
        {
            let def = groups.get_mut(def_id).unwrap();

            let mut damage = atk_power;
            if def.weak.contains(&atk_type) {
                damage *= 2;
            }
            let loss = damage / def.hp;
            def.units = def.units.saturating_sub(loss);
        }
    }
}

pub mod part1 {
    use crate::{Team, battle, parse};

    pub fn solve(file: String) -> usize {
        let mut groups = parse(file);
        loop {
            battle(&mut groups);
            groups.retain(|g| g.units > 0);
            if groups.iter().all(|g| g.team == Team::Immune)
                || groups.iter().all(|g| g.team == Team::Infection)
            {
                break;
            }
        }
        groups.iter().map(|g| g.units).sum::<usize>()
    }
}

pub mod part2 {
    use crate::{Team, battle, parse};

    pub fn solve(file: String) -> usize {
        let origin_groups = parse(file);
        for i in 1.. {
            let mut groups = origin_groups.clone();
            groups = groups.into_iter().map(|mut g| {
                if g.team == Team::Immune {
                    g.atk += i;
                }
                g
            }).collect::<Vec<_>>();
            loop {
                let units = groups.iter().map(|g| g.units).sum::<usize>();
                battle(&mut groups);
                groups.retain(|g| g.units > 0);
                // units non change, skip this round
                if units == groups.iter().map(|g| g.units).sum() {
                    break;
                }
                if groups.iter().all(|g| g.team == Team::Infection)
                {
                    break;
                } else if groups.iter().all(|g| g.team == Team::Immune)
                {
                    return groups.iter().map(|g| g.units).sum();
                }
            }
        }
        0
    }
}
