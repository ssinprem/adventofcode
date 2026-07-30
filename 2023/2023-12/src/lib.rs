use std::collections::HashMap;

fn count_arrangements(
    cfg: &[char],
    nums: &[usize],
    cfg_idx: usize,
    nums_idx: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&res) = memo.get(&(cfg_idx, nums_idx)) {
        return res;
    }

    if nums_idx == nums.len() {
        if cfg.iter().skip(cfg_idx).any(|&c| c == '#') {
            return 0;
        }
        return 1;
    }

    if cfg_idx >= cfg.len() {
        return 0;
    }

    let mut res = 0;

    if cfg[cfg_idx] == '.' || cfg[cfg_idx] == '?' {
        res += count_arrangements(cfg, nums, cfg_idx + 1, nums_idx, memo);
    }

    if cfg[cfg_idx] == '#' || cfg[cfg_idx] == '?' {
        let group_len = nums[nums_idx];
        if cfg_idx + group_len <= cfg.len() {
            let can_form_group = !cfg.iter().skip(cfg_idx).take(group_len).any(|&c| c == '.');

            if can_form_group {
                if cfg_idx + group_len == cfg.len() {
                    res += count_arrangements(cfg, nums, cfg_idx + group_len, nums_idx + 1, memo);
                } else if cfg[cfg_idx + group_len] != '#' {
                    res += count_arrangements(cfg, nums, cfg_idx + group_len + 1, nums_idx + 1, memo);
                }
            }
        }
    }

    memo.insert((cfg_idx, nums_idx), res);
    res
}

pub fn get_possible(record: String, list: Vec<usize>) -> u64 {
    let mut memo = HashMap::new();
    let record_chars: Vec<char> = record.chars().collect();
    count_arrangements(&record_chars, &list, 0, 0, &mut memo)
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        file.lines()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    let (record, list) = line.split_once(" ").unwrap();
                    let list = list
                        .split(",")
                        .map(|str| str.parse::<usize>().unwrap())
                        .collect();
                    Some(get_possible(record.to_string(), list))
                }
            })
            .sum::<u64>()
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        file.lines()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    let (record, list) = line.split_once(" ").unwrap();
                    let record = [record; 5].join("?");
                    let list = [list; 5].join(",")
                        .split(",")
                        .map(|str| str.parse::<usize>().unwrap())
                        .collect();
                    Some(get_possible(record.to_string(), list))
                }
            })
            .sum::<u64>()
    }
}
