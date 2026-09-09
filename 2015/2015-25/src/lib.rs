pub fn position_order (row: u64,col: u64) -> u64 {
    let mut r = 1;
    let mut c = 1;
    let mut max = 1;
    
    for i in 0.. {
        if r == row && c == col {
            return i;
        }
        if r == 1 {
            max += 1;
            r = max;
            c = 1
        } else {
            r -= 1;
            c += 1;
        }
    }
    return 0;
}

pub mod part1 {
    use super::*;

    pub fn solve(row: u64, col: u64) -> u64 {
        let order = position_order(row, col);
        let mut number: u64 = 20151125;
        const MUL: u64 = 252533;
        const MOD: u64 = 33554393;

        for _ in 0..order {
            number *= MUL;
            number %= MOD;
        }
        number
    }
}
