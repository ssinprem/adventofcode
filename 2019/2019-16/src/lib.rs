pub fn get_multipy(round: usize, size: usize) -> Vec<isize> {
    let set = [
        [0_isize].repeat(round),
        [1_isize].repeat(round),
        [0_isize].repeat(round),
        [-1_isize].repeat(round),
    ]
    .concat();

    let mut total = vec![];
    while total.len() < size + 1 {
        total = [total, set.clone()].concat();
    }

    total.into_iter().skip(1).take(size).collect()
}

pub mod part1 {
    use crate::get_multipy;

    pub fn solve(file: String) -> u64 {
        let input = file
            .chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect::<Vec<isize>>();
        let size = input.len();
        let mut num = input.clone();
        for _ in 0..100 {
            let temp = num.clone();

            num = (1..=size)
                .map(|r| {
                    temp.iter()
                        .zip(get_multipy(r, size))
                        .map(|(n, m)| n * m)
                        .sum::<isize>()
                        .abs()
                        % 10
                })
                .collect::<Vec<isize>>();
        }

        num.into_iter()
            .take(8)
            .reduce(|acc, n| acc * 10 + n)
            .unwrap() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
