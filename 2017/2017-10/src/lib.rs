
pub fn parse(file: String) -> Vec<usize> {
    file.split(",").map(|str| {
        str.trim().parse::<usize>().unwrap()
    }).collect::<Vec<usize>>()
}

pub mod part1 {
    use super::parse;
    
    pub fn solve(file: String, len: usize) -> usize {
        let seqs = parse(file);
        let mut index = 0;
        let mut list = (0..len).collect::<Vec<usize>>();
        for (skip, seq) in seqs.into_iter().enumerate() {
            let mut temp = Vec::new();
            for i in 0..seq {
                temp.push(*list.get((index + i) % len).unwrap());
            }
            for i in 0..seq {
                list[(index + i) % len] = temp.pop().unwrap();
            }
            index = (index + seq + skip) % len;
        }
        list[0] * list[1]
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
