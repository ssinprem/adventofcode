use std::collections::VecDeque;

pub fn parse(file: String) -> (usize, usize) {
    use regex::Regex;
    // 10 players; last marble is worth 1618 points
    let regex = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let caps = regex.captures(file.as_str()).unwrap();
    (
        caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
    )
}

pub fn solve(players: usize, turns: usize) -> u64 {
    let mut line = VecDeque::with_capacity(turns);
    let mut cur_player = 1;

    let mut scores = vec![0; players + 1];

    line.push_back(0);
    for turn in 1..=turns {
        if !turn.is_multiple_of(23) {
            if line.len() == 1 {
                line.push_back(turn);
            } else {
                line.rotate_left(1);
                line.push_back(turn);
            }
        } else {
            line.rotate_right(7);
            let removed: usize = line.pop_back().unwrap();
            line.rotate_left(1);
            scores[cur_player] += (turn + removed) as u64;
        }

        cur_player = if cur_player == players {
            1
        } else {
            cur_player + 1
        };
    }

    *scores.iter().max().unwrap()
}
pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (players, turns) = parse(file);
        crate::solve(players, turns)
    }
}

pub mod part2 {
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (players, turns) = parse(file);
        crate::solve(players, turns * 100)
    }
}
