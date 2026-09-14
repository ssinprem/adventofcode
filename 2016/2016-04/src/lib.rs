use regex::Regex;

fn parse(file: String) -> Vec<(String, u64, String)> {
    let regex = Regex::new(r"([a-z-]+)(\d+)\[([a-z]+)\]").unwrap();
    
    file.lines()
    .filter_map(|line| {
        if let Some(caps) = regex.captures(line) 
        && let Some(name) = caps.get(1)
        && let Some(num) = caps.get(2)
        && let Ok(num) = num.as_str().parse::<u64>()
        && let Some(encyp) = caps.get(3)
        {
            Some((
                name.as_str().to_string(),
                num,
                encyp.as_str().to_string()
            ))
        } else {
            None
        }
    }).collect()
}

fn is_real_room(room: &(String, u64, String)) -> bool {
    let encypt = room.2.to_string();
    let name = room.0.to_string();

    let mut list = Vec::new();
    for char in encypt.chars() {
        list.push((char, name.chars().filter(|c| *c == char).count()))
    }
    if list.iter().any(|(_char,num)| *num==0) {
        return false;
    }
    list.windows(2).all(|pair| {
        ( pair[0].1 > pair[1].1 ) ||
        ( 
            pair[1].1 == pair[0].1
            && (pair[0].0 as u8) < (pair[1].0 as u8)
        )
    })
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let rooms = parse(file);

        rooms.iter()
            .filter(|room| is_real_room(room))
            .map(|room| room.1)
            .sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
