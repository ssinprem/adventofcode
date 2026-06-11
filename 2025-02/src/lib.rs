pub mod part1 {
    pub fn solve(file: String) -> u64 {
        file.split(",").map(|range| {
            if let Some((start,end)) =  range.replace("\n", "").split_once("-") {
                let nstart = start.parse::<u64>().unwrap();
                let nend = end.parse::<u64>().unwrap();
                (nstart..=nend).filter(|n| !check_valid(*n)).sum()
            } else {
                0
            }
        }).sum()
    }
    
    fn check_valid(id: u64) -> bool {
        let string = format!("{id}");
        let (front,back) = string.split_at(string.len()/2);
        front != back
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        file.split(",").map(|range| {
            if let Some((start,end)) =  range.replace("\n", "").split_once("-") {
                let nstart = start.parse::<u64>().unwrap();
                let nend = end.parse::<u64>().unwrap();
                (nstart..=nend).filter(|n| !check_valid(*n))
                .inspect(|&f| {println!("{f}");})
                .sum()
            } else {
                0
            }
        }).sum()
    }
    
    fn check_valid(id: u64) -> bool {
        let string = format!("{id}");
        let max_len = string.len();
        (1..=max_len/2).all(|len| {
            let (start,_) = string.split_at(len);
            let text = start.repeat(max_len/len);
            text != string
        })
    }
}
