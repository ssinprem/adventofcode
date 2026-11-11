pub type P = (isize, isize, isize);

pub fn parse(file: String) -> Vec<(P, usize)> {
    let regex = regex::Regex::new(r"pos=<([-0-9]+),([-0-9]+),([-0-9]+)>, r=(\d+)").unwrap();
    file.lines()
        .map(|line| {
            let caps = regex.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
            let z = caps.get(3).unwrap().as_str().parse::<isize>().unwrap();
            let r = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            ((x, y, z), r)
        })
        .collect()
}

pub fn range(bot1: &P, bot2: &P) -> usize {
    bot1.0.abs_diff(bot2.0) + bot1.1.abs_diff(bot2.1) + bot1.2.abs_diff(bot2.2)
}

pub mod part1 {
    use crate::{parse, range};

    pub fn solve(file: String) -> usize {
        let bots = parse(file);

        let big_bot = bots.iter().max_by_key(|(_pos, range)| *range).unwrap();
        bots.iter()
            .filter(|bot| {
                let range = range(&big_bot.0, &bot.0);
                range <= big_bot.1
            })
            .count()
    }
}

pub mod part2 {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use crate::{P, parse, range};

    pub fn solve(file: String) -> usize {
        let bots = parse(file);

        let count_in = |lo: &P, size: usize| -> usize {
            bots.iter()
                .filter(|(p, r)| {
                    // nearest point of the cube to this bot
                    let size = size as isize - 1;
                    let nearest = (
                        p.0.clamp(lo.0, lo.0 + size),
                        p.1.clamp(lo.1, lo.1 + size),
                        p.2.clamp(lo.2, lo.2 + size),
                    );
                    range(p, &nearest) <= *r
                })
                .count()
        };

        let min = bots
            .iter()
            .map(|(p, _)| p.0.min(p.1).min(p.2))
            .min()
            .unwrap();
        let max = bots
            .iter()
            .map(|(p, _)| p.0.max(p.1).max(p.2))
            .max()
            .unwrap();
        let mut size = 1_usize;
        while size < (max - min + 1) as usize {
            size *= 2;
        }
        let lo = (min, min, min);
        let mut heap = BinaryHeap::new();

        heap.push((count_in(&lo, size), Reverse(size), lo));
        while let Some((_count, Reverse(size), lo)) = heap.pop() {
            if size == 1 {
                return range(&lo, &(0, 0, 0));
            }
            let half = size / 2;
            for dx in [0, half as isize] {
                for dy in [0, half as isize] {
                    for dz in [0, half as isize] {
                        let c = (lo.0 + dx, lo.1 + dy, lo.2 + dz);
                        heap.push((count_in(&c, half), Reverse(half), c));
                    }
                }
            }
        }

        0
    }
}
