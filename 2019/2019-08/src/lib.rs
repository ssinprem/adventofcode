pub fn parse(file: String, size: (usize, usize)) -> Vec<Vec<String>> {
    let line = file
        .chars()
        .collect::<Vec<char>>()
        .chunks(size.0)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();

    line
        .chunks(size.1)
        .map(|layer| layer.to_vec())
        .collect::<Vec<_>>()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String, size: (usize, usize)) -> usize {

        let layers = parse(file, size);

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
    use crate::parse;

    pub fn solve(file: String, size: (usize, usize)) -> String {
        let layers = parse(file, size);

        let mut seen = vec![vec!['2'; size.0]; size.1];

        for layer in layers {
            for (y,line) in layer.iter().enumerate() {
                for (x,char) in line.chars().enumerate() {
                    if seen[y][x] == '2' {
                        let pix = seen.get_mut(y).unwrap().get_mut(x).unwrap();
                        *pix = char;
                    }
                }
            }
        }

        seen.iter().map(|line| 
            line.iter().map(|char| {
                match *char {
                    '1' => '#',
                    '0' => ' ',
                    _ => '?'
                }
            }).collect::<String>()
        ).collect::<Vec<String>>().join("\n")
    }
}
