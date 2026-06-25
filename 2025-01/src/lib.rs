pub mod part1 {
    pub fn solution1 (file: String) -> u32 {
        let mut cur_pos = 50;
        let mut cnt_zero = 0;
        for line in file.lines() {
            if line.starts_with("L") {
                cur_pos -= line.split("L").last().unwrap().parse().unwrap_or(0);
            } else if line.starts_with("R") {
                cur_pos += line.split("R").last().unwrap().parse().unwrap_or(0)
            }
    
            if cur_pos % 100 == 0 {
                cnt_zero += 1;
            }
        }
        cnt_zero
    }
    
    pub fn solution2 (file: String) -> u32 {
        let mut cur_pos = 50;
        let mut cnt_zero = 0;
        file.lines().for_each(|line| {
            cur_pos += match line {
                l if line.starts_with("L") => {
                    -l.split("L").last().unwrap().parse::<i32>().unwrap()
                }
                l if line.starts_with("R") => {
                    l.split("R").last().unwrap().parse::<i32>().unwrap()
                }
                _ => 0
            };
            cur_pos = ((cur_pos % 100) + 100) % 100;
            if cur_pos == 0 {
                cnt_zero += 1;
            }
        });
        cnt_zero
    }
    
    pub fn solution3 (file: String) -> u32 {
        file.lines().fold((50, 0), |(mut curr, mut cnt), line| {
            curr += match line {
                l if l.starts_with("L") => {
                    -l[1..].parse::<i32>().unwrap()
                }
                r if r.starts_with("R") =>{
                    r[1..].parse::<i32>().unwrap()
                }
                _ => 0
            };
            if curr % 100 == 0 {
                cnt += 1;
            }
            (curr, cnt)
        }).1
    }
    
    pub fn solution4 (file: String) -> u32 {
        let mut cur = 50;
        file.lines()
        .map(|line| {
            if let Some(str_num) = line.strip_prefix('L') {
                -str_num.parse::<i32>().unwrap()
            } else if let Some(str_num) = line.strip_prefix('R') {
                str_num.parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .map(|val| {
            cur += val;
            cur
        })
        .filter(|v| *v % 100 == 0 )
        .count() as u32
    }
}


pub mod part2 {
    #[allow(clippy::manual_unwrap_or)]
    #[allow(clippy::manual_unwrap_or_default)]
    pub fn solution1 (file: String) -> u32 {
        file.lines().fold(
            (50, 0), 
            |(mut curr, mut cnt), line| {
                let num = 
                if let Ok(num) = line.strip_prefix("L").unwrap_or("!").parse::<i32>() {
                    - num
                } else if let Ok(num) = line.strip_prefix("R").unwrap_or("!").parse::<i32>() {
                    num
                } else {
                    0
                };
                if num > 0 {
                    for _ in 0..num {
                        curr += 1;
                        if curr == 100 {
                            curr = 0;
                        }
                        if curr == 0 {
                            cnt += 1;
                        }
                    }
                } else {
                    for _ in num..0 {
                        curr -= 1;
                        if curr == -1 {
                            curr = 99;
                        }
                        if curr == 0 {
                            cnt += 1;
                        }
                    }
                }
                (curr, cnt)
            }
        ).1
    }
}