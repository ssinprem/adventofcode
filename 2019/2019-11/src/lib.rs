use std::collections::HashMap;

use crate::Dir::*;

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    N,
    E,
    W,
    S,
}

pub struct Robot {
    dir: Dir,
    pos: (isize, isize),
    is_move_next: bool,
}

impl Robot {
    fn turn(&mut self, inst: usize) {
        // inst 0 - turn left 90
        //      1 - turn right 90
        self.dir = match (inst, self.dir.clone()) {
            (0, N) => W,
            (0, E) => N,
            (0, S) => E,
            (0, W) => S,
            (1, N) => E,
            (1, E) => S,
            (1, S) => W,
            (1, W) => N,
            _ => unreachable!(),
        };
    }

    fn walk(&mut self) {
        self.pos = match self.dir {
            N => (self.pos.0, self.pos.1 - 1),
            E => (self.pos.0 + 1, self.pos.1),
            S => (self.pos.0, self.pos.1 + 1),
            W => (self.pos.0 - 1, self.pos.1),
        };
    }
}

pub fn process(
    memory: &mut HashMap<usize, i128>,
    pc: &mut usize,
    relative_base: &mut isize,
    input: i128,
) -> Option<usize> {
    loop {
        let inst = memory.get(pc).expect("cannot access pc mem");
        let p1_mode = (inst / 100) % 10;
        let p2_mode = (inst / 1000) % 10;
        let p3_mode = (inst / 10000) % 10;

        let a_value = if ![1, 2, 4, 5, 6, 7, 8, 9].contains(&(inst % 100)) {
            0
        } else if p1_mode == 0 {
            let addr = *memory.get(&(*pc + 1)).unwrap() as usize;
            *memory.get(&addr).unwrap_or(&0)
        } else if p1_mode == 1 {
            *memory.get(&(*pc + 1)).unwrap_or(&0)
        } else if p1_mode == 2 {
            let offset = *memory.get(&(*pc + 1)).unwrap() as isize;
            *memory
                .get(&((*relative_base + offset) as usize))
                .unwrap_or(&0)
        } else {
            unreachable!()
        };
        let b_value = if ![1, 2, 5, 6, 7, 8].contains(&(inst % 100)) {
            0
        } else if p2_mode == 0 {
            let addr = *memory.get(&(*pc + 2)).unwrap() as usize;
            *memory.get(&addr).unwrap_or(&0)
        } else if p2_mode == 1 {
            *memory.get(&(*pc + 2)).unwrap_or(&0)
        } else if p2_mode == 2 {
            let offset = *memory.get(&(*pc + 2)).unwrap() as isize;
            *memory
                .get(&((*relative_base + offset) as usize))
                .unwrap_or(&0)
        } else {
            unreachable!()
        };

        let write_addr = if ![1, 2, 3, 7, 8].contains(&(inst % 100)) {
            0
        } else if inst % 100 == 3 {
            if p1_mode == 0 {
                *memory.get(&(*pc + 1)).unwrap() as usize
            } else if p1_mode == 2 {
                (*relative_base + (*memory.get(&(*pc + 1)).unwrap()) as isize) as usize
            } else {
                unreachable!()
            }
        } else {
            if p3_mode == 0 {
                *memory.get(&(*pc + 3)).unwrap() as usize
            } else if p3_mode == 2 {
                (*relative_base + (*memory.get(&(*pc + 3)).unwrap()) as isize) as usize
            } else {
                unreachable!()
            }
        };
        if inst % 100 == 1 {
            // Add
            let result = a_value + b_value;
            memory.insert(write_addr, result);
            *pc += 4;
        } else if inst % 100 == 2 {
            // Mul
            let result = a_value * b_value;
            memory.insert(write_addr, result);
            *pc += 4;
        } else if inst % 100 == 3 {
            // Input
            memory.insert(write_addr, input);
            *pc += 2;
        } else if inst % 100 == 4 {
            // Output
            *pc += 2;
            return Some(a_value as usize);
            // output.push(format!("{a_value}"));
        } else if inst % 100 == 5 {
            // Jump if true
            if a_value != 0 {
                *pc = b_value as usize
            } else {
                *pc += 3;
            }
        } else if inst % 100 == 6 {
            // Jump if false
            if a_value == 0 {
                *pc = b_value as usize
            } else {
                *pc += 3;
            }
        } else if inst % 100 == 7 {
            // less than  ,, true put 1
            memory.insert(write_addr, if a_value < b_value { 1 } else { 0 });
            *pc += 4;
        } else if inst % 100 == 8 {
            // equal ,, true put 1
            memory.insert(write_addr, if a_value == b_value { 1 } else { 0 });
            *pc += 4;
        } else if inst % 100 == 9 {
            // move relative
            *relative_base += a_value as isize;
            *pc += 2;
        } else if inst % 100 == 99 {
            return None;
        } else {
            unreachable!();
        }
    }
}

pub fn painting_bot(inst: usize, robot: &mut Robot, paint: &mut HashMap<(isize, isize), usize>) {
    if robot.is_move_next {
        // turn and walk
        robot.turn(inst);
        robot.walk();
        robot.is_move_next = false;
    } else {
        // paint
        paint.insert(robot.pos, inst);
        robot.is_move_next = true;
    }
}

fn _display(paint: &HashMap<(isize, isize), usize>, cursor: Option<&Robot>) {
    let mut min_x = paint.iter().min_by_key(|((t, _), _)| *t).unwrap().0.0;
    let mut max_x = paint.iter().max_by_key(|((t, _), _)| *t).unwrap().0.0;
    let mut min_y = paint.iter().min_by_key(|((_, t), _)| *t).unwrap().0.1;
    let mut max_y = paint.iter().max_by_key(|((_, t), _)| *t).unwrap().0.1;

    if let Some(robot) = cursor {
        min_x = min_x.min(robot.pos.0);
        max_x = max_x.max(robot.pos.0);
        min_y = min_y.min(robot.pos.1);
        max_y = max_y.max(robot.pos.1);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(robot) = cursor
                && robot.pos == (x, y)
            {
                print!(
                    "{}",
                    match robot.dir {
                        N => "^",
                        E => ">",
                        S => "v",
                        W => "<",
                    }
                );
            } else if let Some(c) = paint.get(&(x, y)) {
                if *c == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::{_display, Dir::*, Robot, painting_bot, process};

    pub fn solve(file: String) -> usize {
        let mut memory = aoc_2019_09::parse(file);
        let mut relative_base = 0;
        let mut pc = 0;
        let mut paint: HashMap<(isize, isize), usize> = HashMap::new();
        let mut robot = Robot {
            pos: (0, 0),
            dir: N,
            is_move_next: false,
        };

        loop {
            let input = paint.get(&robot.pos).unwrap_or(&0);
            if let Some(output) = process(&mut memory, &mut pc, &mut relative_base, *input as i128)
            {
                painting_bot(output, &mut robot, &mut paint);
            } else {
                break;
            }
        }

        _display(&paint, None);

        paint.len()
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use crate::{_display, Dir::*, Robot, painting_bot, process};

    pub fn solve(file: String) -> usize {
        let mut memory = aoc_2019_09::parse(file);
        let mut relative_base = 0;
        let mut pc = 0;
        let mut paint: HashMap<(isize, isize), usize> = HashMap::new();
        let mut robot = Robot {
            pos: (0, 0),
            dir: N,
            is_move_next: false,
        };
        paint.insert((0,0), 1);

        loop {
            let input = paint.get(&robot.pos).unwrap_or(&0);
            if let Some(output) = process(&mut memory, &mut pc, &mut relative_base, *input as i128)
            {
                painting_bot(output, &mut robot, &mut paint);
            } else {
                break;
            }
        }

        _display(&paint, None);

        paint.len()
    }
}
