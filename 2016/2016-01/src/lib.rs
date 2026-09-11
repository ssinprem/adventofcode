pub mod part1 {
    pub fn solve(file: String) -> u64 {
        let mut head = (0, 1); // N E S W
        let mut pos: (i32, i32) = (0, 0);

        let steps = file
            .clone()
            .split(",")
            .map(|step| {
                let (h, n) = step.trim().split_at(1);
                let n = n.parse::<u32>().unwrap();
                (h.chars().next().unwrap(), n)
            })
            .collect::<Vec<(char, u32)>>();

        for (h, n) in steps {
            if h == 'L' {
                head = match head {
                    (0,1) /* N */=> (-1,0), /* W */
                    (1,0) /* E */=> (0,1), /* N */
                    (0,-1) /* S */=> (1,0), /* E */
                    (-1,0) /* W */=> (0,-1), /* S */
                    _ => unreachable!()
                }
            } else if h == 'R' {
                head = match head {
                    (0,1) /* N */=> (1,0), /* E */
                    (1,0) /* E */=> (0,-1), /* S */
                    (0,-1) /* S */=> (-1,0), /* W */
                    (-1,0) /* W */=> (0,1), /* N */
                    _ => unreachable!()
                }
            }
            for _ in 0..n {
                pos.0 += head.0;
                pos.1 += head.1;
            }
        }

        (pos.0.abs() + pos.1.abs()) as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
