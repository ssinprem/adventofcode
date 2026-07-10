
// Numpad
//     -2  -1   0
//    +---+---+---+
// -3 | 7 | 8 | 9 |
//    +---+---+---+
// -2 | 4 | 5 | 6 |
//    +---+---+---+
// -1 | 1 | 2 | 3 |
//    +---+---+---+
//  0     | 0 | A |
//        +---+---+

// directional keypad
//    -2  -1   0
//       +---+---+
// 0     | ^ | A |
//   +---+---+---+
// 1 | < | v | > |
//   +---+---+---+

pub fn get_positon(char: char) -> (isize,isize) {
    match char {
        'A' => (0, 0),
        '0' => (-1,0),
        '1' => (-2,-1),
        '2' => (-1,-1),
        '3' => (0,-1),
        '4' => (-2,-2),
        '5' => (-1,-2),
        '6' => (0,-2),
        '7' => (-2,-3),
        '8' => (-1,-3),
        '9' => (0,-3),
        '<' => (-2,1),
        'v' => (-1,1),
        '>' => (0,1),
        '^' => (-1,0),
        _ => todo!("not implement")
    }
}

pub fn get_seq_by_btn(from: char, to: char) -> String {
    let from_pos = get_positon(from);
    let to_pos = get_positon(to);
    let mut cur_pos = from_pos;
    

    match (from,to) {
        ('A','^') => {return "<A".to_string()},
        ('A','<') => {return "v<<A".to_string()},
        ('A','v') => {return "v<A".to_string()},
        ('A','>') => {return "vA".to_string()},
        ('<','A') => {return ">>^A".to_string()},
        ('^','A') => {return ">A".to_string()},
        ('v','A') => {return ">^A".to_string()},
        ('>','A') => {return "^A".to_string()},
        ('<','^') => {return ">^A".to_string()},
        _ => {}
    }

    let mut out = String::from("");
    // Priority ^ > v <
    let mut direction = 0;
    let mut next_pos ;
    let mut next_step= "";
    // println!("{from} {to} -> ");
    while cur_pos != to_pos {
        // println!("{cur_pos:2?} {direction}");
        next_pos = cur_pos;
        if direction == 3 && cur_pos.1 < to_pos.1 {
            next_pos = (cur_pos.0, cur_pos.1 + 1);
            next_step = "v";
        }
        if direction == 1 && cur_pos.1 > to_pos.1 {
            next_pos = (cur_pos.0, cur_pos.1 - 1);
            next_step = "^";
        }
        if direction == 2 && cur_pos.0 > to_pos.0  {
            next_pos = (cur_pos.0 - 1, cur_pos.1);
            next_step = "<";
        }
        if direction == 0 && cur_pos.0 < to_pos.0 {
            next_pos = (cur_pos.0 + 1, cur_pos.1);
            next_step = ">";
        }

        // check invalid position (-2,0)
        if next_pos != cur_pos && next_pos != (-2,0) {
            // println!("{cur_pos:2?} {direction} {next_step} {next_pos:?}");
            cur_pos = next_pos;
            out += next_step;
        } else {
            direction = (direction + 1) % 4;
        }
    }
    out+"A"
}

pub fn get_seq_by_set(set: String) -> String {
    // initial cursor at 'A' (0,0)
    let seq = "A".to_string() + set.as_str();

    seq.chars().collect::<Vec<char>>().windows(2)
    .fold("".to_string(),|str, pair| {
        str + get_seq_by_btn(pair[0], pair[1]).as_str()
    })
}

pub mod part1 {
    use crate::get_seq_by_set;

    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .map(|line| {
            let num = line.strip_suffix("A").unwrap()
            .parse::<u64>().unwrap();
            let mut temp = get_seq_by_set(line.to_string());
            println!("{temp}");
            temp = get_seq_by_set(temp);
            println!("{temp}");
            temp = get_seq_by_set(temp);
            println!("{temp}");
            let result = num * temp.len() as u64;
            println!("{result} -- [{}] {num} {temp}", temp.len());
            result
        }).sum()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
