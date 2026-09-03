
pub mod part1 {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};

    pub fn solve(file: String) -> HashMap<String, u16> {
        let mut hm: HashMap<String, u16> = HashMap::new();
        let mut hs = HashSet::new();

        let regex = Regex::new(r"(.*) -> (\w+)").unwrap();
        let regex_node = Regex::new(r"(\w+)").unwrap();
        let regex_number = Regex::new(r"(\d+)").unwrap();
        let regex_andor = Regex::new(r"(\w+|\d+) (AND|OR) (\w+|\d+)").unwrap();
        let regex_shift = Regex::new(r"(\w+) (L|R)SHIFT (\d+)").unwrap();
        let regex_not = Regex::new(r"NOT (\w+)").unwrap();
        loop {
            file.lines().filter(|line| !line.is_empty())
            .for_each(|line| {
                if let Some(cap) = regex.captures(line)
                 && let Some(left) = cap.get(1)
                 && let Some(right) = cap.get(2)
                {
                    let res = right.as_str().to_string();
                    hs.insert(res.to_string());

                    if hm.get(&res).is_some() {
                        // none action
                    }
                    else if let Some(cap) = regex_andor.captures(left.as_str())
                    && let Some(w1) = cap.get(1)
                    && let Some(op) = cap.get(2)
                    && let Some(w2) = cap.get(3)
                    {
                        let v1 = if let Some(v1) = hm.get(w1.as_str()) {
                            *v1
                        } else if let Ok(v1) = w1.as_str().parse::<u16>() {
                            v1
                        } else {
                            return
                        };
                        let v2 = if let Some(v1) = hm.get(w2.as_str()) {
                            *v1
                        } else if let Ok(v1) = w2.as_str().parse::<u16>() {
                            v1
                        } else {
                            return
                        };
                        if op.as_str() == "AND" {
                            let val = v1 & v2;
                            hm.insert(res.to_string(), val);
                        } else if op.as_str() == "OR" {
                            let val = v1 | v2;
                            hm.insert(res.to_string(), val);
                        }
                    }
                    else if let Some(cap) = regex_shift.captures(left.as_str())
                    && let Some(w) = cap.get(1)
                    && let Some(op) = cap.get(2)
                    && ["L","R"].contains(&op.as_str())
                    && let Some(num_str) = cap.get(3)
                    && let Ok(n) = num_str.as_str().parse::<u16>()
                    {
                        let v = if let Some(v) = hm.get(w.as_str()) {
                            *v
                        } else if let Ok(v) = w.as_str().parse::<u16>() {
                            v
                        } else {
                            return
                        };
                        if op.as_str() == "L" {
                            let val = v << n;
                            hm.insert(res.to_string(), val);
                        } else if op.as_str() == "R" {
                            let val = v >> n;
                            hm.insert(res.to_string(), val);
                        }
                    }
                    else if let Some(cap) = regex_not.captures(left.as_str())
                    && let Some(w) = cap.get(1)
                    && let Some(v) = hm.get(w.as_str())
                    {
                        let val = ! *v;
                        hm.insert(res.to_string(), val);
                    } 
                    else if let Some(cap) =  regex_number.captures(left.as_str()) 
                    && let Some(num_str) = cap.get(1)
                    && let Ok(n) = num_str.as_str().parse::<u16>()
                    {
                        hm.insert(res.to_string(), n);
                    }
                    else if let Some(cap) =  regex_node.captures(left.as_str()) 
                    && let Some(w) = cap.get(1)
                    && let Some(v) = hm.get(w.as_str())
                    {
                        hm.insert(res.to_string(), *v);
                    }
                }
            });

            if hm.len() ==  hs.len() {
                break;
            }
        }
        hm
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
