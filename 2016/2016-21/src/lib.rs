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
                if dir == "right" {
                    chars.rotate_right(n);
                } else if dir == "left" {
                    chars.rotate_left(n);
                } else {
                    unreachable!();
                }
            }
            else if let Some(cap) =
                Regex::new(r"rotate based on position of letter (\w)")
                .unwrap().captures(step)
            && let Some(a) = cap.get(1)
            && let Some(a) = a.as_str().chars().next()
            && let Some(a) = chars.iter().position(|&c| c==a)
            {
                chars.rotate_right(a+1);
                if a >= chars.len()-1 {
                    chars.rotate_right(1);
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
                let mut mid = chars[a..=b].to_vec();
                mid.reverse();
                chars = [chars[0..a].to_vec(), mid , chars[b+1..].to_vec()].concat()
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
