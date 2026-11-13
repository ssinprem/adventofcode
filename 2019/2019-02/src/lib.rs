pub fn parse(file: String) -> Vec<usize> {
    file.split(",")
        .map(|str| str.parse::<usize>().unwrap())
        .collect()
}

pub fn process(memory: &[usize]) -> Vec<usize> {
    let mut memory = memory.to_vec();
    let mut pc = 0;
    loop {
        let inst = memory[pc];
        if inst == 1 {
            let a_pos = memory[pc + 1];
            let b_pos = memory[pc + 2];
            let c_pos = memory[pc + 3];

            let a_value = memory[a_pos];
            let b_value = memory[b_pos];

            let result = a_value + b_value;
            memory[c_pos] = result;
            pc += 4;
        } else if inst == 2 {
            let a_pos = memory[pc + 1];
            let b_pos = memory[pc + 2];
            let c_pos = memory[pc + 3];

            let a_value = memory[a_pos];
            let b_value = memory[b_pos];

            let result = a_value * b_value;
            memory[c_pos] = result;
            pc += 4;
        } else if inst == 99 {
            break;
        } else {
            unreachable!();
        }
    }
    memory
}

pub mod part1 {
    use crate::{parse, process};

    pub fn solve(file: String) -> usize {
        let mut memory = parse(file);

        memory[1] = 12;
        memory[2] = 2;

        *process(&memory).first().unwrap()
    }
}

pub mod part2 {
    use crate::{parse, process};
    const TARGET: usize = 19690720;

    pub fn solve(file: String) -> usize {
        let memory = parse(file);

        for i in 0..=99 {
            for j in 0..=99 {
                let mut temp = memory.clone();
                temp[1] = i;
                temp[2] = j;
                let temp = process(&temp);
                if temp[0] == TARGET {
                    return i * 100 + j;
                }
            }
        }
        0
    }
}
