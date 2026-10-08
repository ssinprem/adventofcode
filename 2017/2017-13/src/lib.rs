use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<usize, u32> {
    let mut list = HashMap::new();
    file.lines().for_each(|line| {
        let (id, num) = line.split_once(":").unwrap();
        let id = id.trim().parse::<usize>().expect("cannot parse id");
        let num = num.trim().parse::<u32>().expect("cannot parse number");
        list.insert(id, num);
    });
    list
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::parse;

    pub fn solve(file: String) -> u32 {
        let slots = parse(file);
        let mut slots = slots
            .into_iter()
            .map(|(id, max)| (id, (0, true, max)))
            .collect::<HashMap<usize, _>>();

        let mut score = 0;
        let max_id = slots.keys().copied().max().unwrap();
        for index in 0..=max_id {
            if let Some((pos, _, max)) = slots.get(&index)
                && *pos == 0
            {
                score += index as u32 * *max;
            }

            for (pos, inc, max) in slots.values_mut() {
                if *inc {
                    if *pos < (*max - 1) as usize {
                        *pos += 1;
                    } else {
                        *pos -= 1;
                        *inc = false;
                    }
                } else {
                    if *pos > 0 {
                        *pos -= 1;
                    } else {
                        *pos += 1;
                        *inc = true;
                    }
                }
            }
        }

        score
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
