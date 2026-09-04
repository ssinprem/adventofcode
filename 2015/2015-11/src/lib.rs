pub mod part1 {
    use std::collections::{HashSet, VecDeque};

    pub fn is_valid(pwd: String) -> bool {
        pwd.chars().find(|c| "iol".contains(*c)).is_none()

        &&

        pwd.chars().collect::<Vec<char>>()
            .windows(3).any(|c| {
                c[1] as u32 == (c[0] as u32) + 1
                &&
                c[2] as u32 == (c[0] as u32) + 2
            }
        )

        &&

        pwd.chars().collect::<Vec<char>>()
            .windows(2).filter(|c| {
                c[0] == c[1]
            })
            .map(|c| c[0])
            .collect::<HashSet<char>>().len() >= 2
    }

    pub fn next(pwd: String) -> String {
        let mut chars = pwd.chars().collect::<VecDeque<char>>();

        if let Some(pos) = pwd.chars().position(|c| "iol".contains(c)) {
            let mut char = pwd.chars().nth(pos).unwrap();
            let mut left = pwd.to_string();
            let _right = left.split_off(pos);
            char = ((char as u8) + 1) as char;
            left = left + char.to_string().as_str();
            while left.len() < pwd.len() {
                left += "a"
            }
            return left
        } else {
            let mut i = 0;
            while let Some(back) = chars.pop_back() {
                if back == 'z' {
                    i += 1;
                } else {
                    let c = ((back as u8) + 1) as char;
                    chars.push_back(c);
                    break;
                }
            }
    
            for _ in 0..i {
                chars.push_back('a');
            }
        }
        chars.into_iter().collect::<String>()
    }

    pub fn solve(file: String) -> String {
        let mut pwd = file;
        while ! is_valid(pwd.to_string()) {
            pwd = next(pwd.to_string());
        }
        pwd
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
