use chronal_classification::{Opcode, operate};
use go_with_the_flow::parse;

/// Run the program (with reg0 = 0) and yield the value that register 0 is
/// compared against each time the halting check (`eqrr x 0 y`) executes.
/// Register 0 is never read anywhere else, so these are exactly the reg0
/// values that would make the program halt at that point.
fn halting_candidates(file: String) -> impl Iterator<Item = u64> {
    let (ip_reg, insts) = parse(file);
    let (check_ip, cmp_reg) = insts
        .iter()
        .enumerate()
        .find_map(|(i, (op, a, b, _))| match op {
            Opcode::Eqrr if *b == 0 => Some((i, *a as usize)),
            Opcode::Eqrr if *a == 0 => Some((i, *b as usize)),
            _ => None,
        })
        .expect("no eqrr instruction reading register 0");

    let mut reg = vec![0u64; 6];
    std::iter::from_fn(move || {
        while let Some((opcode, a, b, c)) = insts.get(reg[ip_reg] as usize) {
            let ip = reg[ip_reg] as usize;
            let value = reg[cmp_reg];
            reg = operate(&reg, opcode, &[*a, *b, *c]);
            reg[ip_reg] += 1;
            if ip == check_ip {
                return Some(value);
            }
        }
        None
    })
}

pub mod part1 {
    pub fn solve(file: String) -> u64 {
        // First value checked => halts after the fewest instructions.
        crate::halting_candidates(file).next().unwrap()
    }
}

pub mod part2 {
    use std::collections::HashSet;

    pub fn solve(file: String) -> u64 {
        // Values cycle eventually; the last new value before the first repeat
        // halts after the most instructions.
        let mut seen = HashSet::new();
        let mut last = 0;
        for v in crate::halting_candidates(file) {
            if !seen.insert(v) {
                break;
            }
            last = v;
        }
        last
    }
}
