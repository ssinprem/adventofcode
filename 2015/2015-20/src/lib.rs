pub mod part1 {
    pub fn solve(file: String) -> u64 {
        let target = file.parse::<u64>().expect("cannot parse target");
        
        let limit = target as usize / 10;
        let mut houses = vec![0; limit];
        
        for elf in 1..limit {
            for h in (elf..limit).step_by(elf) {
                houses[h] += elf *10;
            }
        }

        houses.iter().position(|p| *p >= target as usize).unwrap() as u64
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        let target = file.parse::<u64>().expect("cannot parse target") as usize;
        let limit = target as usize / 10;
        let mut houses = vec![0; limit];
        for elf in 1..limit {
            for (v,h) in (elf..limit).step_by(elf).enumerate() {
                if v == 50 {
                    break;
                }
                houses[h] += elf *11;
            }
        }

        houses.iter().position(|p| *p >= target as usize).unwrap() as u64
    }
}
