use regex::Regex;

#[allow(clippy::type_complexity)]
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
        // println!("light:   {lights:?}");
        let mut switchs: Vec<Vec<usize>> = switch.split(" ")
        .filter(|sw| !sw.is_empty())
        .map(|mut sw| {
            sw = sw.strip_prefix("(").expect("error not found '('");
            sw = sw.strip_suffix(")").expect("error not found ')'");
            sw.split(",").map(|l| {
                l.parse::<usize>().expect("cannot parse")
            }).collect::<Vec<usize>>()
        }).collect();
        switchs.sort_by_key(|sw| sw.len());
        switchs.reverse();
        // println!("switch:  {switchs:?}");
        let joltages : Vec<i32> = joltage.split(",")
        .map(|l| {
            l.parse::<i32>().expect("cannot parse")
        }).collect();
        // println!("joltage: {joltages:?}");
        if lights.is_empty() || switchs.is_empty() || joltages.is_empty() {
            None
        } else {
            Some((lights,switchs,joltages))
        }
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
                    for (sw, _item) in switchs.iter().enumerate() {
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
    fn _press(mut joltage: Vec<i32>, switch:Vec<usize>) -> Vec<i32> {
        // print!("press {switch:?}    -> {joltage:?}");
        for l in switch {
            joltage[l] -= 1;
        }
        // println!(" {joltage:?}");
        joltage
    }

    fn _undo(mut joltage: Vec<i32>, switch:Vec<usize>) -> Vec<i32> {
        // print!("press {switch:?}    -> {joltage:?}");
        for l in switch {
            joltage[l] += 1;
        }
        // println!(" {joltage:?}");
        joltage
    }


    fn _find_min(joltage: Vec<i32>, switchs: Vec<Vec<usize>>, min: &mut usize, steps: usize ) {
        if steps >= *min {
            // println!("Overmin {min:?} {}",steps.len());
            return;
        }

        if joltage.iter().any(|&j| j<0 ){
            // println!("Dead end {joltage:?} {} ",steps.len());
            return;
        }

        if switchs.len() <= 1 && joltage.iter().all(|&j| j==0) {
            if steps < *min {
                *min = steps;
            }
            // println!("Found solve {} {steps:?}", steps.len());
            return;
        }

        if joltage.iter().enumerate()
        .filter(|&(_index,&value)| value != 0)
        .any(|(index, &_value)| {
            ! switchs.iter().any(|sw| sw.contains(&index))
        }) {
            // Can't be reach
            return;
        }

        for i in 0..switchs.len() {
            let mut new_switch = switchs.clone();
            let target_switch = new_switch.split_off(i);
            if ! target_switch.is_empty() {
                let switch = target_switch.first().unwrap().clone();
                
                _find_min(
                    _press(joltage.clone(), switch), 
                    target_switch, min, steps+1);
            }
        }
    }

    fn backtrack(
        targets: &mut [i32],
        assigned: &mut [Option<i32>],
        button_to_counters: &[Vec<usize>],
        counter_to_buttons: &[Vec<usize>],
        presses_so_far: usize,
        global_min_presses: &mut usize,
    ) {
        // Pruning 1: Early threshold exit
        if presses_so_far >= *global_min_presses {
            return;
        }

        // Dynamic Selection: Find the unsatisfied counter with the FEWEST unassigned buttons
        let mut best_counter = None;
        let mut min_unassigned = usize::MAX;

        for c in 0..targets.len() {
            if targets[c] > 0 {
                let unassigned_count = counter_to_buttons[c]
                    .iter()
                    .filter(|&&b| assigned[b].is_none())
                    .count();

                if unassigned_count < min_unassigned {
                    min_unassigned = unassigned_count;
                    best_counter = Some(c);
                }
            }
        }

        // Process the most constrained counter
        if let Some(c) = best_counter {
            // Pruning 2: If a counter needs joltage but has 0 available buttons left, it's a dead end
            if min_unassigned == 0 {
                return;
            }

            // Pick the first available unassigned button that can help this counter
            let &b = counter_to_buttons[c]
                .iter()
                .find(|&&b| assigned[b].is_none())
                .unwrap();

            // Calculate the maximum possible presses based on all counters this button impacts
            let mut max_presses = i32::MAX;
            for &cnt_idx in &button_to_counters[b] {
                if cnt_idx < targets.len() {
                    max_presses = targets[cnt_idx].min(max_presses);
                }
            }
            if max_presses < 0 || max_presses == i32::MAX {
                max_presses = 0;
            }

            // Try values from maximum impact down to 0
            for count in (0..=max_presses).rev() {
                // Apply changes
                for &cnt_idx in &button_to_counters[b] {
                    if cnt_idx < targets.len() {
                        targets[cnt_idx] -= count;
                    }
                }
                assigned[b] = Some(count);

                // Recurse
                backtrack(
                    targets,
                    assigned,
                    button_to_counters,
                    counter_to_buttons,
                    presses_so_far + count as usize,
                    global_min_presses,
                );

                // Backtrack (Undo changes)
                assigned[b] = None;
                for &cnt_idx in &button_to_counters[b] {
                    if cnt_idx < targets.len() {
                        targets[cnt_idx] += count;
                    }
                }
            }
        } else {
            // Base Case: Every single counter target has successfully reached exactly 0
            // Any remaining unassigned buttons must be pressed 0 times to avoid overshooting
            *global_min_presses = presses_so_far.min(*global_min_presses);
        }
    }
    
    pub fn solve(file: String) -> u64 {
        file.lines().enumerate().map(|(index, line)| {
            if let Some((_light, switchs, joltages)) = crate::parse(line) {
                let mut jolt = joltages.clone();
                let mut min = usize::MAX;
                let num_buttons = switchs.len();
                let num_counters = joltages.len();
                let mut assigned: Vec<Option<_>> = vec![None; num_buttons];

                // Build a reverse lookup: mapping each counter to the buttons that affect it
                let mut counter_to_buttons = vec![Vec::new(); num_counters];
                for (b_idx, counters) in switchs.iter().enumerate() {
                    for &c_idx in counters {
                        if c_idx < num_counters {
                            counter_to_buttons[c_idx].push(b_idx);
                        }
                    }
                }
                
                backtrack(
                    &mut jolt,
                    &mut assigned,
                    &switchs,
                    &counter_to_buttons,
                    0,
                    &mut min
                );
                println!("{}. -> {min}",index+1);
                min
            } else {
                0
            }
            // if let Some((_light, switchs, joltages)) = crate::parse(line) {
            //     let mut min = usize::MAX;
            //     find_min(joltages.clone(), switchs, &mut min, 0);
            //     println!("{}. -> {min}",index+1);
            //     min
            // } else {
            //     0
            // }
        }).sum::<usize>() as u64
    }
}
