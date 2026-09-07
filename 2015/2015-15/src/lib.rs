use regex::Regex;

#[derive(Clone)]
pub struct Menu {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    _calories: i64
}

pub fn parse(file: String) -> Vec<Menu> {
    let regex = Regex::new(r"^\w+\: capacity ([-0-9]+), durability ([-0-9]+), flavor ([-0-9]+), texture ([-0-9]+), calories ([-0-9]+)$").unwrap();
    file.lines().filter_map(|line| {

        if let Some(caps) = regex.captures(line)
        && let Some(cap) = caps.get(1)
        && let Ok(capacity) = cap.as_str().parse::<i64>()
        && let Some(dur) = caps.get(2)
        && let Ok(durability) = dur.as_str().parse::<i64>()
        && let Some(fla) = caps.get(3)
        && let Ok(flavor) = fla.as_str().parse::<i64>()
        && let Some(txt) = caps.get(4)
        && let Ok(texture) = txt.as_str().parse::<i64>()
        && let Some(cal) = caps.get(5)
        && let Ok(_calories) = cal.as_str().parse::<i64>()
        {
            Some(
                Menu {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    _calories
                }
            )
        } else {
            None
        }
        
    }).collect()
}

pub fn scores(menus: &[Menu], amount: Vec<u32>) -> Option<u64> {
    let size = amount.len();

    let scores: [i64; 4] = (0..size).map(|i| {
        let amt = amount[i];
        let menu = menus[i].clone();
        [
            amt as i64 * menu.capacity,
            amt as i64 * menu.durability,
            amt as i64 * menu.flavor,
            amt as i64 * menu.texture
        ]
    }).reduce(|acc, score| {
        [
            acc[0] + score[0],
            acc[1] + score[1],
            acc[2] + score[2],
            acc[3] + score[3]
        ]
    })?;

    if scores.iter().any(|v| *v <= 0) {
        None
    } else {
        Some(scores.iter().map(|v| *v as u64).product::<u64>())
    }
}

fn generate_spoons(num_items: usize, total: u32) -> Vec<Vec<u32>> {
    let mut results = Vec::new();
    let mut current = Vec::new();

    fn backtrack(remaining_items: usize, remaining_total: u32, current: &mut Vec<u32>, results: &mut Vec<Vec<u32>>) {
        if remaining_items == 1 {
            current.push(remaining_total);
            results.push(current.clone());
            current.pop();
            return;
        }

        for amount in 0..=remaining_total {
            current.push(amount);
            backtrack(remaining_items - 1, remaining_total - amount, current, results);
            current.pop();
        }
    }

    backtrack(num_items, total, &mut current, &mut results);
    results
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let menus = parse(file);
        let spoons = generate_spoons(menus.len(), 100);

        spoons.iter().filter_map(|spoon| {
            scores(&menus, spoon.clone())
        }).max().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
