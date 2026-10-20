use regex::Regex;
use std::collections::HashMap;

#[allow(clippy::type_complexity)]
pub fn parse(
    file: String,
) -> (
    String,
    usize,
    HashMap<String, HashMap<u8, (u8, isize, String)>>,
) {
    let begin = Regex::new(r"Begin in state (\w)")
        .unwrap()
        .captures(file.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    let steps = Regex::new(r"Perform a diagnostic checksum after (\d+) steps.")
        .unwrap()
        .captures(file.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let regex_state = Regex::new(r"In state (\w):\n((?:.+\n)+)").unwrap();
    let regex_cond = Regex::new(r"If the current value is (\d):\n((?:\s+\-\s.+\n)+)").unwrap();
    let regex_write = Regex::new(r"Write the value (\d)\.").unwrap();
    let regex_move = Regex::new(r"Move one slot to the (\w+)\.").unwrap();
    let regex_next = Regex::new(r"Continue with state (\w)\.").unwrap();
    let mut states: HashMap<String, HashMap<u8, (u8, isize, String)>> = HashMap::new();
    regex_state.captures_iter(file.as_str()).for_each(|caps| {
        let node = caps.get(1).unwrap().as_str().to_string();
        let str = caps.get(2).unwrap().as_str();
        let stat = regex_cond
            .captures_iter(str)
            .map(|cap| {
                let cond = cap.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let s = cap.get(2).unwrap().as_str();
                let write = regex_write
                    .captures(s)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap();
                let moved = regex_move.captures(s).unwrap().get(1).unwrap().as_str();
                let moved = if moved == "left" {
                    -1
                } else if moved == "right" {
                    1
                } else {
                    unreachable!();
                };
                let next = regex_next
                    .captures(s)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string();

                (cond, (write, moved, next))
            })
            .collect::<HashMap<u8, (u8, isize, String)>>();
        states.insert(node, stat);
    });

    (begin, steps, states)
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::parse;

    pub fn solve(file: String) -> usize {
        let (mut state, steps, nodes) = parse(file);
        let mut pos = 0;
        let mut tape: HashMap<i32, u8> = HashMap::new();

        for _ in 0..steps {
            let val = if let Some(val) = tape.get_mut(&pos) {
                val
            } else {
                tape.insert(pos, 0);
                tape.get_mut(&pos).unwrap()
            };

            let (write, moved, next) = nodes.get(&state).unwrap().get(val).unwrap();
            *val = *write;
            pos += *moved as i32;
            state = next.to_string();
        }

        tape.values().filter(|val| **val == 1).count()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
