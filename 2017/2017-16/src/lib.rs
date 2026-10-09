pub fn do_spin(string: String, n: usize) -> String {
    let (a, b) = string.split_at(string.len() - n);
    b.to_string() + a
}

pub fn do_exchange(string: String, a: usize, b: usize) -> String {
    let mut vec = string.chars().collect::<Vec<char>>();
    vec.swap(a, b);
    vec.iter().collect::<String>()
}

pub fn do_partner(string: String, a: char, b: char) -> String {
    let mut vec = string.chars().collect::<Vec<char>>();
    let a = string.find(a).unwrap();
    let b = string.find(b).unwrap();
    vec.swap(a, b);
    vec.iter().collect::<String>()
}

pub mod part1 {
    use super::*;
    use regex::Regex;

    pub fn solve(file: String, len: usize) -> String {
        let mut list = (0..len)
            .map(|i| (b'a' + (i as u8)) as char)
            .collect::<Vec<char>>();

        let regex = Regex::new(r"(?:s(\d+))|(?:x(\d+)\/(\d+))|(?:p(\w)\/(\w))").unwrap();

        file.split(",").for_each(|inst| {
            if let Some(matched) = regex.captures(inst) {
                if let Some(spin) = matched.get(1)
                    && let Ok(spin) = spin.as_str().parse::<usize>()
                {
                    list = do_spin(list.iter().collect::<String>(), spin)
                        .chars()
                        .collect::<Vec<char>>();
                } else if let Some(pos1) = matched.get(2)
                    && let Some(pos2) = matched.get(3)
                    && let Ok(pos1) = pos1.as_str().parse::<usize>()
                    && let Ok(pos2) = pos2.as_str().parse::<usize>()
                {
                    list = do_exchange(list.iter().collect::<String>(), pos1, pos2)
                        .chars()
                        .collect::<Vec<char>>();
                } else if let Some(char1) = matched.get(4)
                    && let Some(char2) = matched.get(5)
                    && let Some(char1) = char1.as_str().chars().next()
                    && let Some(char2) = char2.as_str().chars().next()
                {
                    list = do_partner(list.iter().collect::<String>(), char1, char2)
                        .chars()
                        .collect::<Vec<char>>();
                } else {
                    unreachable!()
                }
            }
        });

        list.iter().collect::<String>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
