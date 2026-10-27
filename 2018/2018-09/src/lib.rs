pub fn parse(file: String) -> (usize, usize) {
    use regex::Regex;
    // 10 players; last marble is worth 1618 points
    let regex = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();

    let caps = regex.captures(file.as_str()).unwrap();

    (
        caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<usize>().unwrap()
    )
}

pub mod part1 {
    use std::collections::HashMap;

use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (players, turns) = parse(file);

        let mut line = vec![0];

        let mut cur_player = 1;
        let mut cur_pos = 0;

        let mut scores = HashMap::new();

        for turn in 1..=turns {
            if ! turn.is_multiple_of(23) {
                if line.len() == 1 {
                    line.push(turn);
                    cur_pos = 1
                } else {
                    cur_pos = (cur_pos + 2) % line.len();
                    line.insert(cur_pos, turn);
                }

            } else {
                cur_pos = if cur_pos > 7 {
                    cur_pos - 7
                } else {
                    cur_pos + line.len() - 7
                };

                let marble = line.remove(cur_pos) + turn;

                scores.entry(cur_player)
                    .and_modify(|score| *score += marble as u64)
                    .or_insert(marble as u64);
            }

            cur_player = if cur_player == players {
                1
            } else {
                cur_player + 1
            };
        }

        *scores.values().max().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
