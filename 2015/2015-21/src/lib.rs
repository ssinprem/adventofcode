#[derive(Debug)]
pub struct Fighter {
    pub hitpoint: i64,
    pub damage: i64,
    pub armor: i64,
}

pub fn is_fight_win(figter: &Fighter, boss: &Fighter) -> bool {
    let (mut hp_figter, mut hp_boss) = (figter.hitpoint, boss.hitpoint);

    loop {
        hp_boss -= 1.max(figter.damage - boss.armor);
        if hp_boss <= 0 {
            return true;
        }
        hp_figter -= 1.max(boss.damage - figter.armor);
        if hp_figter <= 0 {
            return false;
        }
    }
}

pub fn parse(file: String) -> Fighter {
    let mut boss = Fighter {
        hitpoint: 0,
        damage: 0,
        armor: 0,
    };
    file.lines().for_each(|line| {
        if line.starts_with("Hit Points: ")
            && let Some(str) = line.strip_prefix("Hit Points: ")
            && let Ok(n) = str.parse::<i64>()
        {
            boss.hitpoint = n;
        } else if line.starts_with("Damage: ")
            && let Some(str) = line.strip_prefix("Damage: ")
            && let Ok(n) = str.parse::<i64>()
        {
            boss.damage = n;
        } else if line.starts_with("Armor: ")
            && let Some(str) = line.strip_prefix("Armor: ")
            && let Ok(n) = str.parse::<i64>()
        {
            boss.armor = n;
        }
    });
    boss
}

pub fn generate_fighters(hitpoint: i64) -> Vec<(u64, Fighter)> {
    // Weapons:    Cost  Damage  Armor
    // Dagger        8     4       0
    // Shortsword   10     5       0
    // Warhammer    25     6       0
    // Longsword    40     7       0
    // Greataxe     74     8       0
    let weapons = vec![
        (8, 4, 0), // must buy exactly one weapon
        (10, 5, 0),
        (25, 6, 0),
        (40, 7, 0),
        (74, 8, 0),
    ];
    // Armor:      Cost  Damage  Armor
    // Leather      13     0       1
    // Chainmail    31     0       2
    // Splintmail   53     0       3
    // Bandedmail   75     0       4
    // Platemail   102     0       5
    let armors = vec![
        (0, 0, 0), // optional
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    // Rings:      Cost  Damage  Armor
    // Damage +1    25     1       0
    // Damage +2    50     2       0
    // Damage +3   100     3       0
    // Defense +1   20     0       1
    // Defense +2   40     0       2
    // Defense +3   80     0       3
    let rings = vec![
        (0, 0, 0), // optional
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];
    let mut fighter = Vec::new();
    for weapon in &weapons {
        for armor in &armors {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ring1 != ring2 {
                        fighter.push((
                            weapon.0 + armor.0 + ring1.0 + ring2.0,
                            Fighter {
                                hitpoint,
                                damage: weapon.1 + ring1.1 + ring2.1,
                                armor: armor.2 + ring1.2 + ring2.2,
                            },
                        ));
                    }
                }
            }
        }
    }

    fighter
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String, hitpoint: i64) -> u64 {
        let boss = parse(file);
        let fighters = generate_fighters(hitpoint);

        let best = fighters
            .iter()
            .filter(|(_cost, fighter)| is_fight_win(fighter, &boss))
            .min_by_key(|(cost, _fighter)| *cost)
            .unwrap();

        best.0
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String, hitpoint: i64) -> u64 {
        let boss = parse(file);
        let fighters = generate_fighters(hitpoint);

        let worst = fighters
            .iter()
            .filter(|(_cost, fighter)| !is_fight_win(fighter, &boss))
            .max_by_key(|(cost, _fighter)| *cost)
            .unwrap();

        worst.0
    }
}
