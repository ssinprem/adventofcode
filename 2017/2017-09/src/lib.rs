use crate::State::*;
use std::collections::HashMap;

#[derive(Debug)]
enum State {
    GroupGarbage,
    SingleGarbage,
    GroupTarget,
}

pub fn process(file: String) -> HashMap<String, u32> {
    let mut score = 0;
    let mut ignore = 0;

    let mut index = 0;
    let chars = file.chars().collect::<Vec<char>>();

    let mut dept = 0;
    let mut state = Vec::new();
    while let Some(char) = chars.get(index) {
        match state.last() {
            None => {
                match char {
                    '{' => {
                        state.push(GroupTarget);
                        dept += 1;
                    }
                    '!' => {
                        state.push(SingleGarbage);
                    }
                    '<' => {
                        state.push(GroupGarbage);
                    }
                    '>' => {
                        unreachable!();
                    }
                    '}' => {
                        unreachable!();
                    }
                    _ => {}
                }
                index += 1;
            }
            Some(GroupTarget) => {
                match char {
                    '{' => {
                        state.push(GroupTarget);
                        dept += 1;
                    }
                    '}' => {
                        state.pop();
                        score += dept;
                        dept -= 1;
                    }
                    '<' => {
                        state.push(GroupGarbage);
                    }
                    '!' => {
                        state.push(SingleGarbage);
                    }
                    _ => {}
                }
                index += 1;
            }
            Some(SingleGarbage) => {
                index += 1;
                state.pop();
            }
            Some(GroupGarbage) => {
                match char {
                    '!' => {
                        state.push(SingleGarbage);
                    }
                    '>' => {
                        state.pop();
                    }
                    _ => { ignore += 1; }
                }
                index += 1;
            }
        }
    }
    HashMap::from([
        ("score".to_string(), score),
        ("ignore".to_string(), ignore)
    ])
}
pub mod part1 {
    use super::process;

    pub fn solve(file: String) -> u32 {
        *process(file).get("score").unwrap()
    }
}

pub mod part2 {
    use super::process;

    pub fn solve(file: String) -> u32 {
        *process(file).get("ignore").unwrap()
    }
}
