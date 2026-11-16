pub fn parse(file: String) -> Vec<Vec<(isize, isize)>> {
    file.lines().map(|line| {
        let mut curr = (0,0);
        let mut vec = vec![];
        line.split(",").for_each(|step| {
            let (dir,num) = step.split_at(1);
            let num = num.parse::<usize>().unwrap();
            for _ in 0..num {
                match dir {
                    "U" => curr = (curr.0, curr.1-1),
                    "D" => curr = (curr.0, curr.1+1),
                    "L" => curr = (curr.0-1, curr.1),
                    "R" => curr = (curr.0+1, curr.1),
                    _ => unreachable!()
                }
                vec.push(curr);
            }
        });
        vec
    }).collect()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> usize {
        let lines = parse(file);
        let line_a = lines.first().unwrap();
        let line_b = lines.last().unwrap();

        let cross = line_a.iter()
            .filter(|p| line_b.contains(p))
            .copied()
            .collect::<Vec<_>>();
        
        cross.iter()
        .map(|p| p.0.abs_diff(0) + p.1.abs_diff(0)).min().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
