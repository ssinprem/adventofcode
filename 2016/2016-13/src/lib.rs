use std::collections::HashMap;


pub fn solve(file: String, vals: &mut HashMap<String, i32>) {
    let lines = file.lines()
        .map(|str| str.to_string()).collect::<Vec<String>>();
    let mut pc = 0;
    while pc < lines.len() {
        let line = lines[pc].to_string();
        print!("[{pc}]  {line:10}");
        let ops = line.split_whitespace()
            .map(|str| str.to_string()).collect::<Vec<String>>();
        if ops[0].as_str() == "cpy" {
            let dst: String = ops[2].to_string();
            let val = if let Some(val) = vals.get(&ops[1].to_string()) {
                *val
            } else {
                ops[1].parse::<i32>().unwrap()
            };
            _ = vals.insert(dst, val);
            pc += 1;
        } else if ops[0].as_str() == "inc" {
            let name = ops[1].to_string();
            vals.entry(name).and_modify(|val| *val+=1);
            pc += 1;
        } else if ops[0].as_str() == "dec" {
            let name = ops[1].to_string();
            vals.entry(name).and_modify(|val| *val-=1);
            pc += 1;
        } else if ops[0].as_str() == "jnz" {
            let name = ops[1].to_string();
            let num = ops[2].parse::<isize>().expect("parse number failed");
            let val = if let Some(val) = vals.get(&name) {
                *val
            } else {
                name.parse::<i32>().unwrap()
            };
            if val != 0 {
                pc = (pc as isize + num) as usize;
            } else {
                pc += 1;
            }
        }
        println!(" [{pc}]  {vals:?}");
    }
}

pub mod part1 {
    use std::collections::HashMap;
    pub fn solve(file: String) -> HashMap<String, i32> {
        let mut vals: HashMap<String, i32> = HashMap::<String, i32>::new();
        for key in ["a","b","c","d"] {
            vals.insert(key.to_string(), 0);
        }
        crate::solve(file, &mut vals);
        vals
    }
}

pub mod part2 {
    use std::collections::HashMap;
    pub fn solve(file: String) -> HashMap<String, i32> {
        let mut vals: HashMap<String, i32> = HashMap::<String, i32>::new();
        vals.insert("c".to_string(), 1);
        for key in ["a","b","d"] {
            vals.insert(key.to_string(), 0);
        }
        crate::solve(file, &mut vals);
        vals
    }
}
