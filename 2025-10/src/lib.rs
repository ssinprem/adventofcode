use regex::Regex;
pub fn parse(line : &str) -> Option<(Vec<bool>, Vec<Vec<usize>>, Vec<i32>)> {
    let re_line = Regex::new(
        r"(?m)\[([.#]+)\] (\(.+\) )+\{([0-9,]+)\}"
    ).unwrap();
    if let Some(caps) = re_line.captures(line) {
        // parsing
        let (_full, [light, switch, joltage ]) = caps.extract();
        let lights: Vec<bool> = light.chars().map(|c| {
                match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                }
            }).collect();
        println!("light:   {lights:?}");
        let switchs: Vec<Vec<usize>> = switch.split(" ")
        .filter(|sw| !sw.is_empty())
        .map(|mut sw| {
            sw = sw.strip_prefix("(").expect("error not found '('");
            sw = sw.strip_suffix(")").expect("error not found ')'");
            sw.split(",").map(|l| {
                l.parse::<usize>().expect("cannot parse")
            }).collect::<Vec<usize>>()
        }).collect();
        // switchs.sort_by_key(|sw| sw.len());
        // switchs.reverse();
        println!("switch:  {switchs:?}");
        let joltages : Vec<i32> = joltage.split(",")
        .map(|l| {
            l.parse::<i32>().expect("cannot parse")
        }).collect();
        println!("joltage: {joltages:?}");
        Some((lights,switchs,joltages))
    } else {
        None
    }
}

pub mod part1 {
    fn press(mut light: Vec<bool>, switch:Vec<usize>) -> Vec<bool> {
        for l in switch {
            light[l] = !light[l];
        }
        light
    }

    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .map(|line| {
            if let Some((lights, switchs, _joltage )) = crate::parse(line) {
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
                })
            } else {
                0
            }
        }).sum::<usize>() as u64
    }
}

pub mod part2 {
    fn press(mut joltage: Vec<i32>, switch:Vec<usize>) -> Vec<i32> {
        // print!("press {switch:?}    -> {joltage:?}");
        for l in switch {
            joltage[l] -= 1;
        }
        // println!(" {joltage:?}");
        joltage
    }

    fn find_min(joltage: Vec<i32>, switchs: Vec<Vec<usize>>, min: &mut usize, steps:Vec<Vec<usize>> ) -> Option<usize> {
        if steps.len() > *min {
            // println!("Overmin {min:?} {steps:?}");
            return None;
        }

        if joltage.iter().any(|&j| j<0 ){
            // println!("Dead end {joltage:?} {} {steps:?}",steps.len());
            return None;
        }

        if joltage.iter().all(|&j| j==0) {
            if steps.len() < *min {
                *min = steps.len();
            }
            println!("Found solve {} {steps:?}", steps.len());
            return Some(steps.len());
        }
        let mut nmin = usize::MAX;
        let mut found = false;
        
        for i in 0..switchs.len() {
            let mut new_steps :Vec<Vec<usize>> = steps.clone();
            let mut new_switch = switchs.clone();
            let target_switch = new_switch.split_off(i);
            if ! target_switch.is_empty() {
                let switch = target_switch.first().unwrap().clone();
                new_steps.push(switch.clone());
                if let Some(new_min) = find_min(
                    press(joltage.clone(), target_switch.first().unwrap().clone()), 
                    target_switch, min, new_steps) {
                    nmin = nmin.min(new_min);
                    found = true;
                }
            }
        }

        if found {
            Some(nmin)
        } else {
            None
        }
    }

    pub fn solve(file: String) -> u64 {
        file.lines().map(|line| {
            if let Some((_light, switchs, joltages)) = crate::parse(line) {
                let mut min = usize::MAX;
                if let Some(_rmin) = find_min(joltages.clone(), switchs, &mut min, Vec::new()) {
                    println!("{min}");
                    return min as u64;
                } else {
                    0
                }
            } else {
                0
            }
        }).sum::<u64>()
    }
}
