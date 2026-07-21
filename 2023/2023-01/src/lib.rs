pub mod part1 {
    pub fn solve(file: String) -> u64 {
        file.lines()
            .filter_map(|line| {
                if let Some(a) = line.find(|c: char| c.is_ascii_digit())
                    && let Some(na) = line.chars().nth(a).and_then(|n| n.to_digit(10))
                    && let Some(b) = line.rfind(|c: char| c.is_ascii_digit())
                    && let Some(nb) = line.chars().nth(b).and_then(|n| n.to_digit(10))
                {
                    Some(na * 10 + nb)
                } else {
                    None
                }
            })
            .sum::<u32>() as u64
    }
}

pub mod part2 {

    pub fn digit_to_text(n: u32) -> String {
        match n {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => unreachable!(),
        }
        .to_string()
    }

    pub fn solve(file: String) -> u64 {
        file.lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let text_a_pos = (1..=9)
                    .filter_map(|n| line.find(&digit_to_text(n).to_string()).map(|pos| (pos, n)))
                    .min_by_key(|(pos, _n)| *pos)
                    .unwrap_or((usize::MAX, 0));
                let num_a_pos = line
                    .find(|c: char| c.is_ascii_digit())
                    .unwrap_or(usize::MAX);
                let text_b_pos = (1..=9)
                    .filter_map(|n| {
                        line.rfind(&digit_to_text(n).to_string())
                            .map(|pos| (pos, n))
                    })
                    .max_by_key(|(pos, _n)| *pos)
                    .unwrap_or((usize::MIN, 0));
                let num_b_pos = line
                    .rfind(|c: char| c.is_ascii_digit())
                    .unwrap_or(usize::MIN);

                let a = if text_a_pos.0 < num_a_pos {
                    text_a_pos.1
                } else {
                    line.chars()
                        .nth(num_a_pos)
                        .and_then(|n| n.to_digit(10))
                        .unwrap()
                };

                let b = if text_b_pos.0 > num_b_pos {
                    text_b_pos.1
                } else {
                    line.chars()
                        .nth(num_b_pos)
                        .and_then(|n| n.to_digit(10))
                        .unwrap()
                };

                a * 10 + b
            })
            .sum::<u32>() as u64
    }
}
