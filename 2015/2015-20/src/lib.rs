pub mod part1 {
    pub fn solve(file: String) -> u64 {
        let target = file.parse::<u64>().expect("cannot parse target");
        
        let mut house = 0;
        let mut presents = vec![0];
        loop {
            house += 1;
            let mut pres = 0;
            for elf in 1..=house {
                if house % elf == 0 {
                    pres += elf;
                }
            }
            presents.push(pres);
            // println!("{house} {pres}");
            if pres >= target/10 {
                return house;
            }
        }
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
