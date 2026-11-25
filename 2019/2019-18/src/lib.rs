use grid::Grid;

#[derive(Debug, Default)]
pub enum Cell {
    #[default]
    Space,
    Wall,
    Key(char),
    Door(char),
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
pub struct State {
    step: usize,
    pos: (usize,usize),
    keys: Vec<char>,
}

pub fn parse(file: String) -> (Grid<Cell>, (usize,usize)) {
    let mut walker = (0,0);
    let mut map = Grid::new(0, 0);
    file.lines().enumerate().for_each(|(y,line)| {
        map.push_row(
            line.chars().enumerate().map(|(x,char)| {
                match char {
                    '@' => {
                        walker = (x,y);
                        Cell::Space
                    },
                    '.' => Cell::Space,
                    '#' => Cell::Wall,
                    c if c.is_lowercase() => Cell::Key(c),
                    c if c.is_uppercase() => Cell::Door(c.to_ascii_lowercase()),
                    _ => unreachable!()
                }
            }).collect()
        );
    });
    (map, walker)
}

pub mod part1 {
    use std::collections::{BTreeSet, HashMap};
    use crate::{Cell, State, parse};

    pub fn solve(file: String) -> usize {
        let (map, walker) = parse(file);
        let mut stack = BTreeSet::new();
        let mut best = HashMap::new();
        let keys_len = map.iter().filter(|cell| matches!(cell, Cell::Key(_))).count();
        stack.insert(State {
            step : 0,
            pos : walker,
            keys : vec![]
        });

        while let Some(state) = stack.pop_first() {
            // If better history for state (position, keys holder), ignore this run
            if let Some(score) = best.get(&(state.pos, state.keys.clone()))
            && *score < state.step
            {
                continue;
            } else {
                best.insert((state.pos, state.keys.clone()), state.step);
            }
            
            // Got all keys end journey
            if state.keys.len() == keys_len {
                continue;
            }

            // adjacent cell able to walk only 4 axis, N E W S,
            [(-1,0),(1,0),(0,1),(0,-1)].iter().for_each(|(dx,dy)| {
                let next_pos = (state.pos.0 as isize + dx, state.pos.1 as isize + dy);

                if (0..map.cols() as isize).contains(&next_pos.0)
                && (0..map.rows() as isize).contains(&next_pos.1) {
                    let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
                    let cell = map.get(next_pos.1, next_pos.0).expect("cannot find cell");

                    match cell {
                        Cell::Space => {
                            stack.insert(State { step: state.step+1, pos: next_pos, keys: state.keys.clone() });
                        },
                        Cell::Key(c) => {
                            // found the key, keep in bag
                            // not care ordering, let sort by alphabet
                            let mut new_keys = state.keys.clone();
                            if !new_keys.contains(c) { new_keys.push(*c); }
                            new_keys.sort();

                            stack.insert(State { step: state.step+1, pos: next_pos, keys: new_keys});
                        },
                        Cell::Door(c) => {
                            // able to walk through the door, If got the pair key.
                            if state.keys.contains(c) {
                                stack.insert(State { step: state.step+1, pos: next_pos, keys: state.keys.clone() });
                            }
                        },
                        Cell::Wall => {}
                    }
                }
            });
        }

        best.into_iter()
        .map(|((_pos,keys),step)| (step, keys.clone()))
        .filter(|(_step, keys)| keys.len() == keys_len)
        .min_by_key(|(step,_)| *step).unwrap().0

    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
