use std::collections::HashMap;

pub fn _display(pad: &HashMap<(usize, usize), usize>) {
    let min_x = pad.keys().map(|(x, _y)| *x).min().unwrap();
    let max_x = pad.keys().map(|(x, _y)| *x).max().unwrap();
    let min_y = pad.keys().map(|(_x, y)| *y).min().unwrap();
    let max_y = pad.keys().map(|(_x, y)| *y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(v) = pad.get(&(x, y)) {
                print!("{}", match *v {
                    0 => " ",
                    1 => "|",
                    2 => "#",
                    3 => "_",
                    4 => "@",
                    _ => "?",
                });
            } else {
                print!("?");
            }
        }
        println!();
    }
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::_display;

    pub fn solve(file: String) -> usize {
        let output = aoc_2019_09::part1::solve(file);
        let mut pad = HashMap::new();

        let output = output
            .split(",")
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        for (x, y, v) in output.chunks(3).map(|chk| (chk[0], chk[1], chk[2])) {
            pad.insert((x, y), v);
        }
        _display(&pad);
        pad.values().filter(|v| v == &&2).count()
    }
}

pub mod part2 {
    use std::collections::HashMap;

    pub fn solve(file: String) -> usize {
        
        let mut pad_x = 0;
        let mut ball_x = 0;
        let mut score = 0;
        let mut output = vec![];
        let mut memory = aoc_2019_09::parse(file.to_string());
        memory.insert(0, 2);

        fn get_joystick ( ball_x: isize, pad_x: isize ) -> i128 {
            if ball_x < pad_x {
                -1_i128
            } else if ball_x > pad_x {
                1_i128
            } else {
                0_i128
            }
        }

        let mut pc = 0;
        let mut relative_base = 0;

        while let Some(out) = aoc_2019_11::process(&mut memory, &mut pc, &mut relative_base, get_joystick(ball_x, pad_x)) {
            output.push(out);
            if output.len() == 3 {
                let (x,y,v) = (output[0], output[1], output[2]);
                output.clear();
                if x == -1 && y == 0 {
                    score = v as usize;
                } else if v == 3 {
                    pad_x = x;
                } else if v == 4 {
                    ball_x = x;
                }
            }
        }
        score
    }
}
