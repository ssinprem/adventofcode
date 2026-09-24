pub mod part1 {
    use regex::Regex;
    pub fn solve(file: String, init: String) -> String {
        let mut chars = init.chars().collect::<Vec<char>>();
        for step in file.lines() {
            if let Some(cap) =
                Regex::new(r"swap position (\d+) with position (\d+)")
                .unwrap().captures(step)
            && let Some(a) = cap.get(1)
            && let Ok(a) = a.as_str().parse::<usize>()
            && let Some(b) = cap.get(2)
            && let Ok(b) = b.as_str().parse::<usize>()
            {
                chars.swap(a, b);
            }
            else if let Some(cap) =
                Regex::new(r"swap letter (\w) with letter (\w)")
                .unwrap().captures(step)
            && let Some(a) = cap.get(1)
            && let Some(a) = a.as_str().chars().next()
            && let Some(a) = chars.iter().position(|&c| c==a)
            && let Some(b) = cap.get(2)
            && let Some(b) = b.as_str().chars().next()
            && let Some(b) = chars.iter().position(|&c| c==b)
            {
                chars.swap(a, b);
            }
            else if let Some(cap) =
                Regex::new(r"rotate (left|right) (\d+) step")
                .unwrap().captures(step)
            && let Some(dir) = cap.get(1)
            && let dir = dir.as_str()
            && let Some(n) = cap.get(2)
            && let Ok(n) = n.as_str().parse::<usize>()
            {
                let len = chars.len();
                if len > 0 {
                    if dir == "right" {
                        chars.rotate_right(n % len);
                    } else if dir == "left" {
                        chars.rotate_left(n % len);
                    } else {
                        unreachable!();
                    }
                }
            }
            else if let Some(cap) =
                Regex::new(r"rotate based on position of letter (\w)")
                .unwrap().captures(step)
            && let Some(a) = cap.get(1)
            && let Some(a) = a.as_str().chars().next()
            && let Some(a) = chars.iter().position(|&c| c==a)
            {
                let rot = 1 + a + if a >= 4 { 1 } else { 0 };
                let len = chars.len();
                if len > 0 {
                    chars.rotate_right(rot % len);
                }
            }
            else if let Some(cap) =
                Regex::new(r"reverse positions (\d+) through (\d+)")
                .unwrap().captures(step)
            && let Some(a) = cap.get(1)
            && let Ok(a) = a.as_str().parse::<usize>()
            && let Some(b) = cap.get(2)
            && let Ok(b) = b.as_str().parse::<usize>()
            {
                chars[a..=b].reverse();
            }
            else if let Some(cap) =
                Regex::new(r"move position (\d+) to position (\d+)")
                .unwrap().captures(step)
            && let Some(a) = cap.get(1)
            && let Ok(a) = a.as_str().parse::<usize>()
            && let Some(b) = cap.get(2)
            && let Ok(b) = b.as_str().parse::<usize>()
            {
                let target = chars.remove(a);
                chars.insert(b, target);
            } else {
                unreachable!();
            }
        }
        
        chars.into_iter().collect::<String>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
