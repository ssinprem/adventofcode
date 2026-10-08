pub mod part1 {

    pub fn solve(file: String) -> usize {
        (0..128).map(|row| {
            let key = format!("{file}-{row}");
            let hash = knot_hash::part2::solve(key.to_string());
            let binary = hash.chars().map(|char| {
                match char {
                    '0' => "0000",
                    '1' => "0001",
                    '2' => "0010",
                    '3' => "0011",
                    '4' => "0100",
                    '5' => "0101",
                    '6' => "0110",
                    '7' => "0111",
                    '8' => "1000",
                    '9' => "1001",
                    'a' => "1010",
                    'b' => "1011",
                    'c' => "1100",
                    'd' => "1101",
                    'e' => "1110",
                    'f' => "1111",
                    _ => unreachable!()
                }
            }).collect::<String>();
            binary.chars().filter(|char| char == &'1').count()
        }).sum::<usize>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
