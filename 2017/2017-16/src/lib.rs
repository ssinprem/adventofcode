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

pub fn do_instuction(string: String, file: String) -> String {
    use regex::Regex;
    let regex = Regex::new(r"(?:s(\d+))|(?:x(\d+)\/(\d+))|(?:p(\w)\/(\w))").unwrap();
    let mut string = string;
    file.split(",").for_each(|inst| {
        if let Some(matched) = regex.captures(inst) {
            if let Some(spin) = matched.get(1)
                && let Ok(spin) = spin.as_str().parse::<usize>()
            {
                string = do_spin(string.to_string(), spin);
            } else if let Some(pos1) = matched.get(2)
                && let Some(pos2) = matched.get(3)
                && let Ok(pos1) = pos1.as_str().parse::<usize>()
                && let Ok(pos2) = pos2.as_str().parse::<usize>()
            {
                string = do_exchange(string.to_string(), pos1, pos2);
            } else if let Some(char1) = matched.get(4)
                && let Some(char2) = matched.get(5)
                && let Some(char1) = char1.as_str().chars().next()
                && let Some(char2) = char2.as_str().chars().next()
            {
                string = do_partner(string.to_string(), char1, char2);
            } else {
                unreachable!()
            }
        }
    });
    string
}
pub mod part1 {
    use super::*;

    pub fn solve(file: String, len: usize) -> String {
        let string = (0..len)
            .map(|i| (b'a' + (i as u8)) as char)
            .collect::<String>();
        do_instuction(string, file)
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String, len: usize) -> String {
        let mut string = (0..len)
            .map(|i| (b'a' + (i as u8)) as char)
            .collect::<String>();

        let original = string.to_string();
        let mut history = Vec::new();
        history.push(string.to_string());
        loop {
            string = do_instuction(string.to_string(), file.clone());
            if string == original {
                break;
            }
            history.push(string.to_string());
        }
        let target_round = 1_000_000_000 % history.len();
        history.get(target_round ).unwrap().to_string()
    }
}
