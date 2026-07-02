fn get_list(file: String) -> (Vec<i32>, Vec<i32>) {
    file.lines().filter(|line| !line.is_empty()).fold(
        (Vec::<i32>::new(), Vec::<i32>::new()),
        |(mut list1, mut list2), line| {
            let mut spliter = line.split_whitespace();
            let left = spliter.next().expect("cannot split left");
            let right = spliter.next().expect("cannot split right");
            list1.push(left.parse().expect("cannot parse left"));
            list2.push(right.parse().expect("cannot parse right"));
            (list1, list2)
        },
    )
}

pub mod part1 {
    pub fn solve(file: String) -> u64 {
        let mut pairs = crate::get_list(file);
        pairs.0.sort();
        pairs.1.sort();
        pairs
            .0
            .iter()
            .zip(pairs.1)
            .map(|(left, right)| (*left - right).abs())
            .sum::<i32>() as u64
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        let pairs = crate::get_list(file);
        pairs
            .0
            .iter()
            .map(|&left| left * pairs.1.iter().filter(|&right| left == *right).count() as i32)
            .sum::<i32>() as u64
    }
}
