pub mod part1 {
    use regex::Regex;

    fn press(mut light: Vec<bool>, switch:Vec<usize>) -> Vec<bool> {
        for l in switch {
            light[l] = !light[l];
        }
        light
    }

    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .map(|line| {
            let re_line = Regex::new(
                r"(?m)\[([.#]+)\] (\(.+\) )+\{([0-9,]+)\}"
            ).unwrap();

            if let Some(caps) = re_line.captures(line) {
                let (_full, [light, switch, _joltage ]) = caps.extract();
                let lights: Vec<bool> = light.chars().map(|c| {
                    match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    }
                }).collect();
                // println!("light:   {lights:?}");
                let switchs: Vec<Vec<usize>> = switch.split(" ")
                .filter(|sw| !sw.is_empty())
                .map(|mut sw| {
                    sw = sw.strip_prefix("(").expect("error not found '('");
                    sw = sw.strip_suffix(")").expect("error not found ')'");
                    sw.split(",").map(|l| {
                        l.parse::<usize>().expect("cannot parse")
                    }).collect::<Vec<usize>>()
                }).collect();
                // println!("switch:  {switchs:?}");
                // println!("joltage: {joltage}");
            
                (1..2_i32.pow(switchs.len() as u32))
                .fold(switchs.len(), |min, sws| {
                    let mut temp_light = lights.clone();
                    for sw in 0..switchs.len() {
                        if (sws >> sw) % 2 == 1 {
                            temp_light = press(temp_light, switchs[sw].clone());
                        }
                    }
                    if temp_light.iter().all(|&l| !l) {
                        sws.count_ones().min(min as u32) as usize
                    } else {
                        min
                    }
                }) as u64
            } else {
                0
            }
        }).sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
