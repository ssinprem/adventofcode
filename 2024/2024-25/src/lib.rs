pub fn parse(file: String) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for pattern in file.split("\n\n") {
        let mut array = [0, 0, 0, 0, 0];
        let mut typed = "none";
        for (l, line) in pattern.lines().enumerate() {
            if l == 0 {
                if line == "#####" {
                    typed = "lock"
                } else if line == "....." {
                    typed = "key"
                }
            } else {
                for (j, char) in line.chars().enumerate() {
                    if typed == "lock" && char == '#' {
                        array[j] = l;
                    }
                    if typed == "key" && char == '.' {
                        array[j] = l;
                    }
                }
            }
        }
        if typed == "lock" {
            locks.push(array);
        }
        if typed == "key" {
            array = array.map(|n| 5 - n);
            keys.push(array);
        }
    }
    (keys, locks)
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (keys, locks) = parse(file);
        let mut count = 0;

        println!("locks: {locks:?}");
        println!("keys: {keys:?}");

        for lock in locks {
            // let expect_key = lock.map(|n| 5-n);
            println!("locks {lock:?}  ");
            for key in keys.iter() {
                print!("  key {key:?} ");
                if (0..5).all(|n| lock[n] + key[n] <= 5) {
                    println!(" ✅ fit");
                    count += 1;
                } else {
                    println!(" ❌ overlap");
                }
            }
        }
        count
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
