pub fn step1(data: String) -> String {
    let a = data.to_string();
    let mut chars = a.chars().collect::<Vec<char>>();
    chars.reverse();
    let b = chars.iter().map(|n| {
        match n {
            '0' => '1',
            '1' => '0',
            _ => unreachable!()
        }
    }).collect::<String>();
    a + "0" + b.as_str() 
}

pub fn step2(data: String) -> String {
    let mut temp = data;
    while temp.len().is_multiple_of(2) {
        temp = temp.chars().collect::<Vec<char>>()
            .chunks(2)
            .map(|pairs| {
                if pairs[0] == pairs[1] {
                    '1'
                } else {
                    '0'
                }
            }).collect::<String>()
    }
    temp
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String, lenght: usize) -> String {
        let mut val = file.to_string();
        while val.len() < lenght {
            val = step1(val.to_string());
        }
        _ = val.split_off(lenght);
        step2(val.to_string())
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
