pub mod part1 {
    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .map(|line| {
            let n: Vec<u64> = line.split("x")
                .map(|n| n.parse::<u64>().unwrap()).collect();
            let l = n[0];
            let w = n[1];
            let h = n[2];

            let peices = [l*w,w*h,h*l];

            2 * peices.iter().sum::<u64>() + peices.iter().min().unwrap()
        }).sum()
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .map(|line| {
            let mut n: Vec<u64> = line.split("x")
                .map(|n| n.parse::<u64>().unwrap()).collect();
            n.sort();
            let l = n[0];
            let w = n[1];
            let h = n[2];

            l*w*h + 2*l + 2*w
        }).sum()
    }
}
