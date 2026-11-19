use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<usize, i128> {
    file.split(",")
        .map(|str| str.parse::<i128>().unwrap())
        .enumerate()
        .collect()
}

pub fn process(memory: &HashMap<usize,i128>, input: i128) -> (HashMap<usize,i128>, String) {
    let mut memory = memory.clone();
    let mut output = Vec::new();
    let mut relative_base = 0_isize;
    let mut pc = 0;
    loop {
        let inst = memory.get(&pc).expect("cannot access pc mem");
        let p1_mode = (inst / 100) % 10;
        let p2_mode = (inst / 1000) % 10;
        let p3_mode = (inst / 10000) % 10;
        if p3_mode != 0 {
            // p3 never be in immediate mode
            unreachable!()
        }
        
        let a_value = if ![1,2,4,5,6,7,8,9].contains(&(inst % 100)) {
            0
        } else if p1_mode == 0 {
            let addr = *memory.get(&(pc+1)).unwrap() as usize;
            *memory.get(&addr).unwrap_or(&0)
        } else if p1_mode == 1 {
            *memory.get(&(pc+1)).unwrap_or(&0)
        } else {
            let offset = *memory.get(&(pc+1)).unwrap() as isize;
            *memory.get(&((relative_base + offset) as usize)).unwrap_or(&0)
        };
        let b_value = if ![1,2,5,6,7,8].contains(&(inst % 100)) {
            0
        } else if p2_mode == 0 {
            let addr = *memory.get(&(pc+2)).unwrap() as usize;
            *memory.get(&addr).unwrap_or(&0)
        } else if p2_mode == 1 {
            *memory.get(&(pc+2)).unwrap_or(&0)
        } else {
            let offset = *memory.get(&(pc+2)).unwrap() as isize;
            *memory.get(&((relative_base + offset) as usize)).unwrap_or(&0)
        };
        if inst % 100 == 1 {
            // Add
            let result = a_value + b_value;
            let p3 = memory[&(pc + 3)] as usize;
            memory.insert(p3, result);
            pc += 4;
        } else if inst % 100 == 2 {
            // Mul
            let result = a_value * b_value;
            let p3 = memory[&(pc + 3)] as usize;
            memory.insert(p3, result);
            pc += 4;
        } else if inst % 100 == 3 {
            // Input
            let a_pos = memory[&(pc + 1)] as usize;
            memory.insert(a_pos, input);
            pc += 2;
        } else if inst % 100 == 4 {
            // Output
            output.push(format!("{a_value}"));
            pc += 2;
        } else if inst % 100 == 5 {
            // Jump if true
            if a_value != 0 {
                pc = b_value as usize
            } else {
                pc += 3;
            }
        } else if inst % 100 == 6 {
            // Jump if false
            if a_value == 0 {
                pc = b_value as usize
            } else {
                pc += 3;
            }
        } else if inst % 100 == 7 {
            // less than  ,, true put 1
            let p3 = memory[&(pc + 3)] as usize;
            memory.insert(p3, if a_value < b_value { 1 } else { 0 });
            pc += 4;
        } else if inst % 100 == 8 {
            // equal ,, true put 1
            let p3 = memory[&(pc + 3)] as usize;
            memory.insert(p3, if a_value == b_value { 1 } else { 0 });
            pc += 4;
        } else if inst % 100 == 9 {
            // move relative
            relative_base += a_value as isize;
            pc += 2;
        } else if inst % 100 == 99 {
            break;
        } else {
            unreachable!();
        }
    }
    (memory, output.join(","))
}

pub mod part1 {
    use crate::{parse, process};

    pub fn solve(file: String) -> String {
        let memory = parse(file);
        let signal = 1;
        let (_, output) = process(&memory, signal);

        output
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
