pub mod part1 {
    use std::collections::HashMap;

    pub fn solve(file: String, a: u64, b: u64) -> (u64, u64) {
        let mut pc: isize = 0;
        let mut register = HashMap::new();
        register.insert("a".to_string(), a);
        register.insert("b".to_string(), b);
        let lines: Vec<String> = file
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();

        while pc < lines.len() as isize {
            let inst = &lines[pc as usize];
            let inst = inst
                .split_terminator(&[' ', ','])
                .filter(|str| !str.is_empty())
                .map(|str| str.to_string())
                .collect::<Vec<String>>();
            match inst[0].as_str() {
                "hlf" => {
                    let opr = &inst[1];
                    let reg = register
                        .get_mut(&opr.to_string())
                        .expect("cannot get register");
                    *reg /= 2;
                    pc += 1;
                }
                "tpl" => {
                    let opr = &inst[1];
                    let reg = register
                        .get_mut(&opr.to_string())
                        .expect("cannot get register");
                    *reg *= 3;
                    pc += 1;
                }
                "inc" => {
                    let opr = &inst[1];
                    let reg = register
                        .get_mut(&opr.to_string())
                        .expect("cannot get register");
                    *reg += 1;
                    pc += 1;
                }
                "jmp" => {
                    let n = &inst[1];
                    let n = n.parse::<isize>().expect("cannot parse offset");
                    pc += n;
                }
                "jie" => {
                    let opr = &inst[1];
                    let reg = register.get(&opr.to_string()).expect("cannot get register");
                    let n = &inst[2];
                    let n = n.parse::<isize>().expect("cannot parse offset");
                    if reg.is_multiple_of(2) {
                        pc += n;
                    } else {
                        pc += 1;
                    }
                }
                "jio" => {
                    let opr = &inst[1];
                    let reg = register.get(&opr.to_string()).expect("cannot get register");
                    let n = &inst[2];
                    let n = n.parse::<isize>().expect("cannot parse offset");
                    if reg == &1 {
                        pc += n;
                    } else {
                        pc += 1;
                    }
                }
                _ => {}
            }
        }

        (*register.get("a").unwrap(), *register.get("b").unwrap())
    }
}
