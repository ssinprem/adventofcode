pub mod part1 {
    pub fn solve(file: String) -> usize {
        let size = file.parse::<usize>().expect("cannot parse size");
        let mut elfs = Vec::new();
        for i in 0..size {
            elfs.push((i+1, 1));
        }

        while elfs.len() > 1 {
            let size = elfs.len();
            for i in 0..size {
                if elfs[i].1 > 0 { // elf with present only
                    if i != size-1 { // not last
                        elfs[i].1 += elfs[i+1].1;
                        elfs[i+1].1 = 0;
                    } else { // is last
                        elfs[i].1 += elfs[0].1;
                        elfs[0].1 = 0;
                    }
                }
            }
            elfs = elfs.iter()
                .filter(|(_id,presents)| *presents>0)
                .copied()
                .collect::<Vec<_>>();
        }
        elfs.first().unwrap().0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
