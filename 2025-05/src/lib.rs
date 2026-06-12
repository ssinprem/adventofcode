pub mod part1 {
    pub fn solve(file: String) -> u64 {
        if let Some((igd, order)) = file.split_once("\n\n") {
            order.lines().map(|line| {
                if let Ok(n) = line.parse::<u64>() {
                    if igd.lines().any(|line| {
                        let (start,end) = line.split_once('-').unwrap();
                        let nstart = start.parse::<u64>().unwrap();
                        let nend = end.parse::<u64>().unwrap();
                        n >= nstart && n <= nend 
                    }) {
                        return 1;
                    }
                } 
                0
            }).sum()
        } else {
            0
        }
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {file.len() as u64}
}
