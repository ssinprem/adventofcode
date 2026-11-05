use chronal_classification::{Opcode};

pub fn parse(file: String) -> (usize,Vec<(Opcode, u64, u64, u64)>) {
    use regex::Regex;
    let first = file.lines().next().unwrap();
    (
        first.split_once(" ").unwrap().1.parse::<usize>().unwrap(),
        file.lines().skip(1).map(|line| {
            let regex = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();

            if let Some(caps) = regex.captures(line) 
            && let Some(opcode_match) = caps.get(1)
            && let Some(a_match) = caps.get(2)
            && let Some(b_match) = caps.get(3)
            && let Some(c_match) = caps.get(4)
            && let Ok(a) = a_match.as_str().parse::<u64>()
            && let Ok(b) = b_match.as_str().parse::<u64>()
            && let Ok(c) = c_match.as_str().parse::<u64>()
            {
                (
                    match opcode_match.as_str() {
                        "addr" => Opcode::Addr,
                        "addi" => Opcode::Addi,
                        "mulr" => Opcode::Mulr,
                        "muli" => Opcode::Muli,
                        "banr" => Opcode::Banr,
                        "bani" => Opcode::Bani,
                        "borr" => Opcode::Borr,
                        "bori" => Opcode::Bori,
                        "setr" => Opcode::Setr,
                        "seti" => Opcode::Seti,
                        "gtir" => Opcode::Gtir,
                        "gtri" => Opcode::Gtri,
                        "gtrr" => Opcode::Gtrr,
                        "eqir" => Opcode::Eqir,
                        "eqri" => Opcode::Eqri,
                        "eqrr" => Opcode::Eqrr,
                        _ => unreachable!()
                    }, a, b, c
                )
            } else {
                unreachable!()
            }
        }).collect()
    )
}

pub mod part1 {
    use chronal_classification::operate;
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (ip_reg, insts) = parse(file);
        let mut reg = vec![0, 0, 0, 0, 0, 0];

        while let Some((opcode, a, b ,c)) = insts.get(reg[ip_reg] as usize) {
            reg = operate(&reg, opcode, &vec![*a,*b,*c]);
            println!("{opcode:?} {a} {b} {c} => {reg:?}");
            reg[ip_reg] += 1;
        }
        reg[0].try_into().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
