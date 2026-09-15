use regex::Regex;

pub fn parse(file: String) -> Vec<(Vec<String>, Vec<String>)> {
    let regex_char = Regex::new(r"\[[^\]]*\]|([^\[\]]+)").unwrap();
    let regex_brackets = Regex::new(r"\[([a-z]+)\]").unwrap();

    file.lines()
        .map(|line| {
            (
                regex_char
                    .captures_iter(line)
                    .filter_map(|cap| cap.get(1).map(|str| str.as_str().to_string()))
                    .collect(),
                regex_brackets
                    .captures_iter(line)
                    .filter_map(|cap| cap.get(1).map(|str| str.as_str().to_string()))
                    .collect(),
            )
        })
        .collect()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let set = parse(file);

        set.iter()
            .filter(|(outside, inside)| {
                outside.iter().any(|string| {
                    string
                        .chars()
                        .collect::<Vec<char>>()
                        .windows(4)
                        .any(|grp| grp[0] == grp[3] && grp[1] == grp[2] && grp[0] != grp[1])
                }) && inside.iter().all(|string| {
                    !string
                        .chars()
                        .collect::<Vec<char>>()
                        .windows(4)
                        .any(|grp| grp[0] == grp[3] && grp[1] == grp[2] && grp[0] != grp[1])
                })
            })
            .count() as u64
    }
}

pub mod part2 {
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let set = parse(file);

        set.iter()
            .filter(|(outside, inside)| {
                for string in inside {
                    let pal = string
                        .chars()
                        .collect::<Vec<char>>()
                        .windows(3)
                        .filter(|chars| chars[0] == chars[2] && chars[0] != chars[1])
                        .map(|chars| format!("{}{}{}", chars[0], chars[1], chars[2]))
                        .collect::<Vec<String>>();

                    for p in pal {
                        let target = format!(
                            "{}{}{}",
                            p.chars().nth(1).unwrap(),
                            p.chars().next().unwrap(),
                            p.chars().nth(1).unwrap()
                        );

                        if outside.iter().any(|out| out.contains(&target)) {
                            return true;
                        }
                    }
                }
                false
            })
            .count() as u64
    }
}
