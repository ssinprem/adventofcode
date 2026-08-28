pub mod part1 {
    pub fn is_valid(text: String) -> bool {
        text.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
        &&
        text.chars().collect::<Vec<char>>()
            .windows(2).any(|cc| cc[0] == cc[1])
        &&
        !["ab","cd","pq","xy"].iter().any(|ban| text.contains(ban))
    }

    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .filter(|line| is_valid(line.to_string()))
        .count() as u64
    }
}

pub mod part2 {
    use std::collections::HashMap;
    pub fn is_valid(text: String) -> bool {
        let pairs = text.chars().collect::<Vec<char>>()
            .windows(2)
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<String>>();

        let hm: HashMap<String, (u64, Vec<usize>)> = pairs.iter().enumerate()
            .fold(HashMap::new(), |mut hm, (pos,string)| {
                hm.entry(string.to_string())
                    .and_modify(|(count, list)| {*count+=1; list.push(pos);})
                    .or_insert((1, vec![pos]));
                hm
            });

        let triples = text.chars().collect::<Vec<char>>()
            .windows(3)
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<String>>();

        hm.values().any(|(count, list)| {
            *count>=2 
            &&
            (0..*count).any(|i| {
                ((i+1)..*count).any(|j| {
                    list[j as usize] - list[i as usize] > 1
                })
            })
        })
        &&
        triples.iter().any(|string| {
            string.chars().nth(0) == string.chars().nth(2)
        })
    }

    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .filter(|line| is_valid(line.to_string()))
        .count() as u64
    }
}
