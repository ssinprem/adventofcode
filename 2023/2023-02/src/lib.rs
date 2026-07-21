use regex::Regex;
use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<u32, Vec<[u32; 3]>> {
    file.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut game = Vec::new();
            let (game_str, detail) = line.split_once(":").unwrap();
            let id_regex = Regex::new(r"Game (\d+)").unwrap();
            let round_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
            let game_id = id_regex
                .captures(game_str)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            for round in detail.split(";") {
                let mut nums = [0, 0, 0];
                for cap in round_regex.captures_iter(round) {
                    if let Some(n) = cap.get(1)
                        && let Ok(num) = n.as_str().to_string().parse::<u32>()
                        && let Some(color) = cap.get(2)
                    {
                        match color.as_str() {
                            "red" => nums[0] = num,
                            "green" => nums[1] = num,
                            "blue" => nums[2] = num,
                            _ => {}
                        }
                    }
                }
                game.push(nums);
            }
            (game_id, game)
        })
        .collect::<HashMap<_, _>>()
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        // limit red 12, green 13, blue 14
        let limit = [12, 13, 14];
        let games = parse(file);

        let mut sum = 0;
        for (id, game) in games.iter() {
            // all round should be in limit
            if game
                .iter()
                .all(|round| (0..3).all(|n| round[n] <= limit[n]))
            {
                // adding by game id, if game is valid
                sum += *id;
            }
        }
        sum.into()
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let games = parse(file);

        games
            .into_values()
            .map(|game| {
                game.iter().map(|r| r[0]).max().unwrap()
                    * game.iter().map(|r| r[1]).max().unwrap()
                    * game.iter().map(|r| r[2]).max().unwrap()
            })
            .sum::<u32>()
            .into()
    }
}
