pub fn parse(file: String) -> (Vec<u64>, Vec<usize>) {
    let mut reg = Vec::new();
    let mut ops = Vec::new();
    file.lines().for_each(|line| {
        if line.starts_with("Register") {
            let (_, val) = line.split_once(":").expect("cannot split register line");
            reg.push(
                val.trim()
                    .parse::<u64>()
                    .unwrap_or_else(|_| panic!("cannot parse register {val}")),
            );
        } else if line.starts_with("Program:") {
            let (_, vals) = line.split_once(":").expect("cannot split program line");
            for val in vals.split(",") {
                ops.push(val.trim().parse::<usize>().expect("cannot parse program"));
            }
        }
    });
    (reg, ops)
}

pub fn run(reg: &Vec<u64>, ops: &Vec<usize>) -> (Vec<u64>, Vec<u64>) {
    let mut output = Vec::new();
    let mut reg = reg.clone();
    let mut pc = 0;
    while pc < ops.len() {
        let opcode = ops[pc];
        let combo_oprand = match ops[pc + 1] {
            x if (0..=3).contains(&x) => x as u64,
            4 => reg[0],
            5 => reg[1],
            6 => reg[2],
            _ => unreachable!(),
        };
        let literal_operand = ops[pc + 1] as u64;

        // println!("PC[{pc:2}] Reg {reg:8?} opcode {opcode} oprand {combo_oprand}, output {output:?}");
        pc += 2;
        match opcode {
            /*adv*/ 0 => reg[0] = reg[0] / 2_u64.pow(combo_oprand as u32) as u64,
            /*bxl*/ 1 => reg[1] ^= literal_operand,
            /*bst*/ 2 => reg[1] = combo_oprand % 8,
            /*jnz*/ 3 => {
                        if reg[0] != 0 {
                            pc = literal_operand as usize;
                            continue;
                        }
                    }
            /*bxc*/ 4 => reg[1] ^= reg[2],
            /*out*/ 5 => output.push(combo_oprand % 8),
            /*bdv*/ 6 => reg[1] = reg[0] / 2_u32.pow(combo_oprand as u32) as u64,
            /*cdv*/ 7 => reg[2] = reg[0] / 2_u32.pow(combo_oprand as u32) as u64,
            _ => unreachable!(),
        }
    }

    // println!("       Reg {reg:8?} output {output:?}");
    (reg, output)
}


pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> String {
        let (reg, ops) = parse(file);
        println!("{reg:?}  {ops:?}");
        let (new_reg,output) = run(&reg,&ops);
        println!("{new_reg:?}  {output:?}");

        output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .to_string()
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let (reg, ops) = parse(file);
        println!("{reg:?}  {ops:?}");
        let (new_reg,output) = run(&reg,&ops);
        println!("{new_reg:?}  {output:?}");
        let mut old_a;
        let mut a = 0;
        loop {
            let (_, new_output) = run(&vec![a,0,0], &ops);
            if new_output.iter().map(|&o| o as u64).collect::<Vec<u64>>()
            == 
            ops.iter().map(|&o| o as u64).collect::<Vec<u64>>() {
            
                return a;
            }
            old_a = a;
            if new_output.len() > ops.len() {
                a /= 2;
            } else if new_output.len() < ops.len() {
                a= a*101/100+1;
            } else if ops.len() > 8 &&
                let Some(x) = (0..10).find(|&b| {
                *new_output.iter().nth_back(b).unwrap() !=
                *ops.iter().nth_back(b).unwrap() as u64
            }) {
                let diff = ops.len() as u64 - x as u64;
                a = a + (3_u32.pow(diff as u32)+1) as u64 * 15
                    
            } else {
                a+=1;
            }
            println!("{a} -> {new_output:?}  diff = {}", a as i64 - old_a as i64);
        };
    }
}
