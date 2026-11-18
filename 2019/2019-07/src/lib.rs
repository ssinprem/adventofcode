pub fn parse(file: String) -> Vec<isize> {
    file.split(",")
        .map(|str| str.parse::<isize>().expect("cannot parse to number"))
        .collect()
}

pub mod part1 {
    use crate::parse;
    use itertools::Itertools;

    pub fn process(memory: &[isize], phase: isize, input: isize) -> (Vec<isize>, isize) {
        let mut first_input = true;
        let mut memory = memory.to_vec();
        let mut output = Vec::new();
        let mut pc = 0;
        loop {
            let inst = memory[pc];
            let p1_mode = ((inst / 100) % 10) == 1;
            let p2_mode = ((inst / 1000) % 10) == 1;
            let p3_mode = ((inst / 10000) % 10) == 1;
            if p3_mode {
                // p3 never be in immediate mode
                unreachable!()
            }
            // println!("[{pc}] {memory:?}");
            if inst % 100 == 1 {
                // Add
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
                // Mul
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
                // Input
                let a_pos = memory[pc + 1];
                let input = if first_input {
                    first_input = false;
                    phase
                } else {
                    input
                };
                memory[a_pos as usize] = input;
                pc += 2;
            } else if inst % 100 == 4 {
                // Output
                let a_pos = memory[pc + 1];
                let a_value = memory[a_pos as usize];
                output.push(format!("{a_value}"));
                pc += 2;
            } else if inst % 100 == 5 {
                // Jump if true
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
                if a_value != 0 {
                    pc = b_value as usize
                } else {
                    pc += 3;
                }
            } else if inst % 100 == 6 {
                // Jump if false
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
                if a_value == 0 {
                    pc = b_value as usize
                } else {
                    pc += 3;
                }
            } else if inst % 100 == 7 {
                // less than  ,, true put 1
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
                let p3 = memory[pc + 3] as usize;
                memory[p3] = if a_value < b_value { 1 } else { 0 };
                pc += 4;
            } else if inst % 100 == 8 {
                // equal ,, true put 1
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
                let p3 = memory[pc + 3] as usize;
                memory[p3] = if a_value == b_value { 1 } else { 0 };
                pc += 4;
            } else if inst == 99 {
                break;
            } else {
                unreachable!();
            }
        }
        (memory, output.join("").parse::<isize>().unwrap())
    }

    pub fn solve(file: String) -> (isize, Vec<isize>) {
        let origin_memory = parse(file);
        let rank = (0..5)
            .permutations(5)
            .map(|phases| {
                let mut signal = 0;
                let mut memory = origin_memory.clone();
                for phase in phases.iter() {
                    (memory, signal) = process(&memory, *phase, signal);
                }
                (signal, phases.clone())
            })
            .collect::<Vec<_>>();

        rank.into_iter().max_by_key(|(output, _)| *output).unwrap()
    }
}

pub mod part2 {
    use crate::parse;
    use itertools::Itertools;

    pub fn process(
        memory: &[isize],
        pc: usize,
        phase: isize,
        input: isize,
        first_input: bool,
    ) -> (Vec<isize>, usize, isize, bool) {
        let mut memory = memory.to_vec();
        let mut pc = pc;
        let mut first_input = first_input;
        loop {
            let inst = memory[pc];
            let p1_mode = ((inst / 100) % 10) == 1;
            let p2_mode = ((inst / 1000) % 10) == 1;
            let p3_mode = ((inst / 10000) % 10) == 1;
            if p3_mode {
                // p3 never be in immediate mode
                unreachable!()
            }
            // println!("[{pc}] {memory:?}");
            if inst % 100 == 1 {
                // Add
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
                // Mul
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
                // Input
                let a_pos = memory[pc + 1];
                let input = if first_input {
                    first_input = false;
                    phase
                } else {
                    input
                };
                memory[a_pos as usize] = input;
                pc += 2;
            } else if inst % 100 == 4 {
                // Output
                let a_pos = memory[pc + 1];
                let a_value = memory[a_pos as usize];
                // output.push(format!("{a_value}"));

                pc += 2;
                let out = a_value;
                return (memory, pc, out, false);
            } else if inst % 100 == 5 {
                // Jump if true
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
                if a_value != 0 {
                    pc = b_value as usize
                } else {
                    pc += 3;
                }
            } else if inst % 100 == 6 {
                // Jump if false
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
                if a_value == 0 {
                    pc = b_value as usize
                } else {
                    pc += 3;
                }
            } else if inst % 100 == 7 {
                // less than  ,, true put 1
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
                let p3 = memory[pc + 3] as usize;
                memory[p3] = if a_value < b_value { 1 } else { 0 };
                pc += 4;
            } else if inst % 100 == 8 {
                // equal ,, true put 1
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
                let p3 = memory[pc + 3] as usize;
                memory[p3] = if a_value == b_value { 1 } else { 0 };
                pc += 4;
            } else if inst == 99 {
                return (memory, 0, 0, true);
            } else {
                unreachable!();
            }
        }
    }

    pub fn solve(file: String) -> (isize, Vec<isize>) {
        let origin_memory = parse(file);
        let rank = (5..10)
            .permutations(5)
            .map(|phases| {
                let mut amps = phases
                    .iter()
                    .map(|p| {
                        (
                            origin_memory.clone(), // memory
                            0,                     // pc
                            *p,                    // phase
                            true,                  // first_input
                            0,                     // last_output
                        )
                    })
                    .collect::<Vec<_>>();

                let mut signal = 0;
                let mut id = 0;
                loop {
                    let (out_memory, pc, sig, halt) =
                        process(&amps[id].0, amps[id].1, amps[id].2, signal, amps[id].3);
                    amps[id].0 = out_memory;
                    amps[id].1 = pc;
                    amps[id].3 = false;
                    amps[id].4 = sig;
                    signal = sig;

                    if halt {
                        break;
                    }
                    id = (id + 1) % amps.len();
                }
                (amps.last().unwrap().4, phases.clone())
            })
            .collect::<Vec<_>>();

        rank.into_iter().max_by_key(|(output, _)| *output).unwrap()
    }
}
