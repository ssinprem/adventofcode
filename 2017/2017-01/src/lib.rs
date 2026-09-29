pub mod part1 {
    pub fn solve(file: String) -> u32 {
        let mut list = file.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

        list.push(*list.first().unwrap());
        list.windows(2).map(|pairs| {
            if pairs[0] == pairs[1] {
                pairs[0]
            } else {
                0
            }
        }).sum()
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u32 {
        let mut front_list = file.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
        let back_list = front_list.split_off(front_list.len()/2);
        front_list.into_iter().zip(back_list)
        .collect::<Vec<(u32,u32)>>()
        .into_iter().map(|(f,b)| {
            if f==b {
                f+b
            } else {
                0
            }
        }).sum()
    }
}
