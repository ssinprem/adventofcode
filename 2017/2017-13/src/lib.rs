use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<usize, usize> {
    let mut list = HashMap::new();
    file.lines().for_each(|line| {
        let (id, num) = line.split_once(":").unwrap();
        let id = id.trim().parse::<usize>().expect("cannot parse id");
        let num = num.trim().parse::<usize>().expect("cannot parse number");
        list.insert(id, num);
    });
    list
}

pub fn process(slots: &mut HashMap<usize, (usize, bool, usize)>) {
    for (pos, inc, max) in slots.values_mut() {
        if *inc {
            if *pos < (*max - 1) as usize {
                *pos += 1;
            } else {
                *pos -= 1;
                *inc = false;
            }
        } else {
            if *pos > 0 {
                *pos -= 1;
            } else {
                *pos += 1;
                *inc = true;
            }
        }
    }
}

pub fn _display(slots: &HashMap<usize, (usize, bool, usize)>) {
    let max_slots = slots.keys().copied().max().unwrap();
    let max_val = slots.values().map(|(_,_,max)| *max).max().unwrap();

    for i in 0..=max_slots {
        if !slots.contains_key(&i) {
            // print!("... ");
        } else {
            print!(" {i}  ");
        }
    }
    println!();

    // line 0
    for i in 0..=max_slots {
        if !slots.contains_key(&i) {
            // print!("... ");
        } else {
            print!("[");
            if slots.get(&i).unwrap().0 == 0 {
                print!("S");
            } else {
                print!(" ");
            }
            print!("] ");
        }
    }
    println!();
    for row in 1..=max_val {
        for i in 0..=max_slots {
            if !slots.contains_key(&i)
            || row >= slots.get(&i).unwrap().2
            {
                // print!("    ");
            } else {
                print!("[");
                if slots.get(&i).unwrap().0 == row {
                    print!("S");
                } else {
                    print!(" ");
                }
                print!("] ");
            }
        }
        println!();
    }

}
pub mod part1 {
    use std::collections::HashMap;

    use crate::{parse, process};

    pub fn solve(file: String) -> usize {
        let slots = parse(file);
        let mut slots = slots
            .into_iter()
            .map(|(id, max)| (id, (0, true, max)))
            .collect::<HashMap<usize, _>>();

        let mut score = 0;
        let max_id = slots.keys().copied().max().unwrap();
        for index in 0..=max_id {
            if let Some((pos, _, max)) = slots.get(&index)
                && *pos == 0
            {
                score += index * *max;
            }
            process(&mut slots);
        }
        score
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use crate::{parse, process};

    pub fn solve(file: String) -> usize {
        let original = parse(file);
        let mut original = original
            .into_iter()
            .map(|(id, max)| (id, (0, true, max)))
            .collect::<HashMap<usize, _>>();
        let max_id = original.keys().copied().max().unwrap();

        for skip in 0.. {
            let mut found = false;
            let mut slots = original.clone();
            process(&mut original);
            for index in 0..=max_id {
                if let Some((pos, _, _)) = slots.get(&index)
                && *pos == 0
                {
                    found = true;
                    break;
                }
                process(&mut slots);
            }

            if found {
                continue;
            } else {
                return skip;
            }
        }
        0
    }
}
