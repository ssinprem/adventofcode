
pub fn parse(file: String) -> Vec<isize> {
    file.split(",")
        .map(|str| str.parse::<isize>().unwrap())
        .collect()
}

pub fn process(memory: &[isize]) -> (Vec<isize>, String) {
    let mut memory = memory.to_vec();
    let mut output = String::new();
    let mut pc = 0;
    loop {
        let inst = memory[pc];
        let p1_mode = ((inst / 100) % 10) == 1;
        let p2_mode = ((inst / 1000) % 10) == 1;
        let p3_mode = ((inst / 10000) % 10) == 1;
        if p3_mode { // p3 never be in immediate mode
            unreachable!()
        }
        if inst % 100 == 1 {
            let a_value = if !p1_mode {
                memory[memory[pc + 1] as usize]
            } else {
                memory[pc + 1]
            };
            let b_value = if !p2_mode {
                memory[memory[pc + 2] as usize]
            } else {
                memory[pc + 2]
            };
            let result = a_value + b_value;
            let p3 = memory[pc + 3] as usize;
            memory[p3] = result;
            pc += 4;
        } else if inst % 100 == 2 {
            let a_value = if !p1_mode {
                memory[memory[pc + 1] as usize]
            } else {
                memory[pc + 1]
            };
            let b_value = if !p2_mode {
                memory[memory[pc + 2] as usize]
            } else {
                memory[pc + 2]
            };
            let result = a_value * b_value;
            let p3 = memory[pc + 3] as usize;
            memory[p3] = result;
            pc += 4;
        } else if inst % 100 == 3 {
            let a_pos = memory[pc + 1];
            memory[a_pos as usize] = 1;
            pc += 2;
        } else if inst % 100 == 4 {
            let a_pos = memory[pc + 1];
            let a_value = memory[a_pos as usize];
            output += format!("{a_value},").as_str();
            pc += 2;
        } else if inst == 99 {
            break;
        } else {
            unreachable!();
        }
    }
    (memory, output)
}

pub mod part1 {
    use crate::{parse, process};

    pub fn solve(file: String) -> String {
        let memory = parse(file);

        let (_memory, output) = process(&memory);
        
        output
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
