#[derive(PartialEq, Clone, Debug)]
pub enum Skill {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

#[derive(Clone, Debug)]
pub struct Wizard {
    hitpoint: i64,
    mana: i64,
    mana_used: i64,
    armor: i64,
    buff: Vec<(Skill, i64)>
}

#[derive(Clone, Debug)]
pub struct Boss {
    hitpoint: i64,
    damage: i64,
    debuff: Vec<(Skill, i64)>
}

pub fn parse(file: String) -> (i64, i64) {
    let mut boss = (0,0);
    file.lines().for_each(|line| {
        if line.starts_with("Hit Points: ")
            && let Some(str) = line.strip_prefix("Hit Points: ")
            && let Ok(n) = str.parse::<i64>()
        {
            boss.0 = n;
        } else if line.starts_with("Damage: ")
            && let Some(str) = line.strip_prefix("Damage: ")
            && let Ok(n) = str.parse::<i64>()
        {
            boss.1 = n;
        }
    });
    boss
}

pub fn action_skill(skill: &Skill,  wizard : &mut Wizard, boss: &mut Boss ) -> bool {
    use crate::Skill::*;
    match skill {
        MagicMissile => {
            if wizard.mana < 53 {
                return false;
            }
            wizard.mana -= 53;
            wizard.mana_used += 53;
            boss.hitpoint -= 4;
        },
        Drain => {
            if wizard.mana < 73 {
                return false;
            }
            wizard.mana -= 73;
            wizard.mana_used += 73;
            wizard.hitpoint += 2;
            boss.hitpoint -= 2;
        },
        Shield => {
            if wizard.mana < 113
            || (wizard.buff.iter().any(|(buff,_turn)| *buff == Shield))
            {
                return false;
            }
            wizard.mana -= 113;
            wizard.mana_used += 113;
            wizard.buff.push((Shield, 6));
        },
        Poison => {
            if wizard.mana < 173
            || (boss.debuff.iter().any(|(buff,_turn)| *buff == Poison))
            {
                return false;
            }
            wizard.mana -= 173;
            wizard.mana_used += 173;
            boss.debuff.push((Poison, 6));
        },
        Recharge => {
            if wizard.mana < 229
            || (wizard.buff.iter().any(|(buff,_turn)| *buff == Recharge))
            {
                return false;
            }
            wizard.mana -= 229;
            wizard.mana_used += 229;
            wizard.buff.push((Recharge, 5));
        },
    }
    true
}

pub fn at_start(wizard : &mut Wizard, boss: &mut Boss) {
    wizard.armor = 0; // by default without shield
    for (buff, left) in wizard.buff.iter_mut() {
        match buff {
            Skill::Shield => wizard.armor = 7,
            Skill::Recharge => wizard.mana += 101,
            _ => {}
        };
        *left -= 1;
    }
    wizard.buff = wizard.buff.clone().into_iter()
        .filter(|(_buff, turn)| *turn>0 )
        .collect::<Vec<(Skill, i64)>>();

    for (debuf, left) in boss.debuff.iter_mut() {
        match debuf {
            Skill::Poison => boss.hitpoint -= 3,
            _ => {}
        };
        *left -= 1;
    }
    boss.debuff = boss.debuff.clone().into_iter()
        .filter(|(_buff, turn)| *turn>0 )
        .collect::<Vec<(Skill, i64)>>();
}

pub mod part1 {
    use crate::*;
    use crate::Skill::*;
    pub fn best_skill_set(wizard : Wizard, boss: Boss) -> (Option<i64>, Vec<Skill>) {
        let mut current_set: Vec<(Wizard, Boss, Vec<Skill>)> = vec![(wizard, boss, vec![])];
        let mut best_skill: Vec<Skill> = Vec::new();
        let mut best_mana = None;
        while let Some((mut wizard,mut boss,skills)) = current_set.pop() {
            if best_mana.is_some() && Some(wizard.mana_used) > best_mana {
                continue;
            }
            // Wizard start turn
            at_start(&mut wizard, &mut boss);
            if boss.hitpoint <= 0 { // boss die
                if best_mana.is_none() || Some(wizard.mana_used) < best_mana {
                    best_mana = Some(wizard.mana_used);
                    best_skill = skills.clone();
                    println!("{best_mana:?} {best_skill:?}");
                }
                continue;
            }
            for skill in [ MagicMissile, Drain, Shield, Poison, Recharge] {
                let mut new_skills = skills.clone();
                let mut new_wizard = wizard.clone();
                let mut new_boss = boss.clone();
                if ! action_skill(&skill, &mut new_wizard, &mut new_boss) {
                    continue;
                }
                new_skills.push(skill.clone());
                // Boss Start
                at_start(&mut new_wizard, &mut new_boss);
                if new_boss.hitpoint <= 0 { // boss die
                    if best_mana.is_none() || Some(new_wizard.mana_used) < best_mana {
                        best_mana = Some(new_wizard.mana_used);
                        best_skill = new_skills.clone();
                        println!("{best_mana:?} {best_skill:?}");
                    }
                    continue;
                }

                new_wizard.hitpoint -= 1.max(new_boss.damage - new_wizard.armor);
                if new_wizard.hitpoint <= 0 { // wizard die
                    continue;
                }
                current_set.push((new_wizard,new_boss,new_skills));
            }
        }

        (best_mana, best_skill)
    }

    pub fn solve(file: String, hitpoint: i64, mana: i64) -> Option<i64> {
        let boss = parse(file);
        let boss = Boss {
            hitpoint: boss.0 as i64,
            damage: boss.1,
            debuff: vec![]
        };
        let wizard = Wizard {
            hitpoint,
            mana,
            armor: 0,
            mana_used: 0,
            buff: vec![]
        };

        let best = best_skill_set(wizard, boss);
        best.0
    }
}

pub mod part2 {
    use crate::*;
    use crate::Skill::*;
    pub fn best_skill_set(wizard : Wizard, boss: Boss) -> (Option<i64>, Vec<Skill>) {
        let mut current_set: Vec<(Wizard, Boss, Vec<Skill>)> = vec![(wizard, boss, vec![])];
        let mut best_skill: Vec<Skill> = Vec::new();
        let mut best_mana = None;
        while let Some((mut wizard,mut boss,skills)) = current_set.pop() {
            if best_mana.is_some() && Some(wizard.mana_used) > best_mana {
                continue;
            }
            // Wizard start turn
            at_start(&mut wizard, &mut boss);
            wizard.hitpoint -= 1;
            if wizard.hitpoint <= 0 {
                continue;
            }
            if boss.hitpoint <= 0 { // boss die
                if best_mana.is_none() || Some(wizard.mana_used) < best_mana {
                    best_mana = Some(wizard.mana_used);
                    best_skill = skills.clone();
                    println!("{best_mana:?} {best_skill:?}");
                }
                continue;
            }
            for skill in [ MagicMissile, Drain, Shield, Poison, Recharge] {
                let mut new_skills = skills.clone();
                let mut new_wizard = wizard.clone();
                let mut new_boss = boss.clone();
                if ! action_skill(&skill, &mut new_wizard, &mut new_boss) {
                    continue;
                }
                new_skills.push(skill.clone());
                // Boss Start
                at_start(&mut new_wizard, &mut new_boss);
                if new_boss.hitpoint <= 0 { // boss die
                    if best_mana.is_none() || Some(new_wizard.mana_used) < best_mana {
                        best_mana = Some(new_wizard.mana_used);
                        best_skill = new_skills.clone();
                        println!("{best_mana:?} {best_skill:?}");
                    }
                    continue;
                }

                new_wizard.hitpoint -= 1.max(new_boss.damage - new_wizard.armor);
                if new_wizard.hitpoint <= 0 { // wizard die
                    continue;
                }
                current_set.push((new_wizard,new_boss,new_skills));
            }
        }

        (best_mana, best_skill)
    }

    pub fn solve(file: String, hitpoint: i64, mana: i64) -> Option<i64> {
        let boss = parse(file);
        let boss = Boss {
            hitpoint: boss.0 as i64,
            damage: boss.1,
            debuff: vec![]
        };
        let wizard = Wizard {
            hitpoint,
            mana,
            armor: 0,
            mana_used: 0,
            buff: vec![]
        };

        let best = best_skill_set(wizard, boss);
        best.0
    }
}
