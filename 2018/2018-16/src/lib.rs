#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

const ALL_OPCODE: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

pub fn operate(reg: &[u64], opcode: &Opcode, oprand: &[u64]) -> Vec<u64> {
    let mut output = reg.to_vec();
    let opr_a = oprand[0];
    let opr_b = oprand[1];
    let opr_c = oprand[2];
    match opcode {
        Opcode::Addr => {
            output[opr_c as usize] = reg[opr_a as usize] + reg[opr_b as usize];
        }
        Opcode::Addi => {
            output[opr_c as usize] = reg[opr_a as usize] + opr_b;
        }
        Opcode::Mulr => {
            output[opr_c as usize] = reg[opr_a as usize] * reg[opr_b as usize];
        }
        Opcode::Muli => {
            output[opr_c as usize] = reg[opr_a as usize] * opr_b;
        }
        Opcode::Banr => {
            output[opr_c as usize] = reg[opr_a as usize] & reg[opr_b as usize];
        }
        Opcode::Bani => {
            output[opr_c as usize] = reg[opr_a as usize] & opr_b;
        }
        Opcode::Borr => {
            output[opr_c as usize] = reg[opr_a as usize] | reg[opr_b as usize];
        }
        Opcode::Bori => {
            output[opr_c as usize] = reg[opr_a as usize] | opr_b;
        }
        Opcode::Setr => {
            output[opr_c as usize] = reg[opr_a as usize];
        }
        Opcode::Seti => {
            output[opr_c as usize] = opr_a;
        }
        Opcode::Gtir => {
            output[opr_c as usize] = if opr_a > reg[opr_b as usize] { 1 } else { 0 };
        }
        Opcode::Gtri => {
            output[opr_c as usize] = if reg[opr_a as usize] > opr_b { 1 } else { 0 };
        }
        Opcode::Gtrr => {
            output[opr_c as usize] = if reg[opr_a as usize] > reg[opr_b as usize] { 1 } else { 0 };
        }
        Opcode::Eqir => {
            output[opr_c as usize] = if opr_a == reg[opr_b as usize] { 1 } else { 0 };
        }
        Opcode::Eqri => {
            output[opr_c as usize] = if reg[opr_a as usize] == opr_b { 1 } else { 0 };
        }
        Opcode::Eqrr => {
            output[opr_c as usize] = if reg[opr_a as usize] == reg[opr_b as usize] { 1 } else { 0 };
        }
    }
    output.to_vec()
}

pub fn parse(file: String) -> (Vec<Vec<Vec<u64>>>, Vec<Vec<u64>>) {
    use regex::Regex;
    let (case_str, inst_str) = file.split_once("\n\n\n").unwrap();
    let regex = Regex::new(r"(\d+)").unwrap();
    (
        case_str
            .split("\n\n")
            .map(|str| {
                str.lines()
                    .map(|line| {
                        regex
                            .captures_iter(line)
                            .map(|nstr| nstr.get(1).unwrap().as_str().parse::<u64>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        inst_str
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|str| str.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

pub mod part1 {
    use crate::{ALL_OPCODE, operate, parse};

    pub fn solve(file: String) -> usize {
        let (cases, _inst) = parse(file);

        cases
            .iter()
            .filter(|case| {
                let before = case[0].clone();
                let inst = case[1].clone();
                let after = case[2].clone();

                ALL_OPCODE
                    .iter()
                    .filter(|opcode| {
                        let output = operate(
                            &before,
                            opcode,
                            &inst.iter().skip(1).copied().collect::<Vec<u64>>(),
                        );

                        output == after
                    })
                    .count()
                    >= 3
            })
            .count()
    }
}

pub mod part2 {
    use crate::{ALL_OPCODE, Opcode, operate, parse};
    use std::collections::{HashMap, HashSet};
    pub fn solve(file: String) -> usize {
        let (cases, insts) = parse(file);

        let mut opcode_convert: HashMap<usize, HashSet<Opcode>> = HashMap::new();
        for c in 0..16 {
            opcode_convert.insert(c, ALL_OPCODE.into_iter().collect::<HashSet<Opcode>>());
        }

        cases.iter().for_each(|case| {
            let before = case[0].clone();
            let inst = case[1].clone();
            let after = case[2].clone();

            let oprand = inst.iter().skip(1).copied().collect::<Vec<u64>>();
            let op_candidate = opcode_convert.get_mut(&(inst[0] as usize)).unwrap();

            let ops = op_candidate.iter().copied().collect::<Vec<Opcode>>();

            for opcode in ops {
                let output = operate(&before, &opcode, &oprand);
                if output != after {
                    op_candidate.remove(&opcode);
                }
            }
        });

        let mut opcode_final = HashMap::new();
        while opcode_final.len() != 16 {
            let opcode_temp = opcode_convert.iter().collect::<Vec<_>>();
            for (code, set) in opcode_temp.iter() {
                if set.len() == 1 {
                    opcode_final.insert(**code, *set.iter().next().unwrap());
                } else {
                    let candidate = set
                        .iter()
                        .filter(|op| !opcode_final.iter().any(|(_c, v)| v == *op))
                        .copied()
                        .collect::<Vec<Opcode>>();
                    if candidate.len() == 1
                        && let Some(opcode) = candidate.into_iter().next()
                    {
                        opcode_final.insert(**code, opcode);
                    }
                }
            }
        }
        println!("{opcode_final:?}");

        let mut reg = vec![0, 0, 0, 0];
        for inst in insts {
            let opcode_n = inst[0];
            let oprand = inst[1..].to_vec();
            let opcode = opcode_final.get(&(opcode_n as usize)).unwrap();
            reg = operate(&reg, opcode, &oprand);
            println!("{opcode:?} {oprand:?} => {reg:?}");
        }

        reg[0].try_into().unwrap()
    }
}
