use regex::Regex;

pub fn parse(file: String) -> Vec<(Vec<String>, Vec<String>)> {
    let regex_char = Regex::new(r"\[[^\]]*\]|([^\[\]]+)").unwrap();
    let regex_brackets = Regex::new(r"\[([a-z]+)\]").unwrap();

    file.lines().map(|line| {
        (
            regex_char.captures_iter(line)
            .filter_map(|cap| {
                cap.get(1).map(|str| str.as_str().to_string())
            }).collect(),
            regex_brackets.captures_iter(line)
            .filter_map(|cap| {
                cap.get(1).map(|str| str.as_str().to_string())
            }).collect()
        )
    }).collect()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let set = parse(file);

        set.iter().filter(|(outside,inside)| {
            outside.iter().any(|string| {
                string.chars().collect::<Vec<char>>().windows(4)
                .any(|grp| 
                    grp[0] == grp[3]
                    && grp[1] == grp[2]
                    && grp[0] != grp[1]
                )
            })
            &&
            inside.iter().all(|string| {
                ! string.chars().collect::<Vec<char>>().windows(4)
                .any(|grp| 
                    grp[0] == grp[3]
                    && grp[1] == grp[2]
                    && grp[0] != grp[1]
                )
            })
        }).count() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
