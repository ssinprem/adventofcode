pub fn is_triangle(len: &[u64]) -> bool {
    let mut len = len.to_vec();
    len.sort();
    len[0] + len[1] > len[2]
}

pub fn parse(file: String) -> Vec<Vec<u64>> {
    file.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|str| str.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

pub mod part1 {
    use crate::{is_triangle, parse};

    pub fn solve(file: String) -> u64 {
        let list = parse(file);
        list.into_iter().filter(|len| is_triangle(len)).count() as u64
    }
}

pub mod part2 {
    use crate::{is_triangle, parse};

    pub fn solve(file: String) -> u64 {
        let mut list = parse(file);

        list = list
            .chunks(3)
            .flat_map(|rows| {
                vec![
                    vec![rows[0][0], rows[1][0], rows[2][0]],
                    vec![rows[0][1], rows[1][1], rows[2][1]],
                    vec![rows[0][2], rows[1][2], rows[2][2]],
                ]
            })
            .collect();

        list.into_iter().filter(|len| is_triangle(len)).count() as u64
    }
}
