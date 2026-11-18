pub mod part1 {
    pub fn solve(file: String, size: (usize, usize)) -> usize {
        let line = file
            .chars()
            .collect::<Vec<char>>()
            .chunks(size.0)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>();

        let layers = line
            .chunks(size.1)
            .map(|layer| layer.to_vec())
            .collect::<Vec<_>>();

        let min = layers
            .iter()
            .min_by_key(|layer| {
                layer
                    .iter()
                    .map(|line| line.chars().filter(|char| char == &'0').count())
                    .sum::<usize>()
            })
            .unwrap();

        let count_1 = min.join("").chars().filter(|c| c == &'1').count();
        let count_2 = min.join("").chars().filter(|c| c == &'2').count();

        count_1 * count_2
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
