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
    use std::collections::VecDeque;

    pub fn _solve(file: String) -> usize {
        let mut size = file.parse::<usize>().expect("cannot parse size");
        let mut elfs = VecDeque::new();
        for i in 0..size {
            elfs.push_back(i+1);
        }

        while size > 1 {
            let oposite = size / 2;
            let target = elfs.remove(oposite).expect("invalid target");
            if size.is_multiple_of(1000) {
                println!("remain {}, {:?} take from {}", size, elfs.front(), target);
            }
            elfs.rotate_left(1);
            size -= 1;
        }
        *elfs.front().unwrap()
    }

    pub fn solve(file: String) -> usize {
        let n = file.parse::<usize>().expect("cannot parse size");
        let mut p = 1;
        while p * 3 <= n {
            p *= 3;
        }

        if n == p {
            n
        } else if n <= 2 * p {
            n - p
        } else {
            2 * n - 3 * p
        }
    }
}
