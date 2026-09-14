pub mod part1 {
    pub fn solve(file: String) -> String {
        let mut n = 1;
        let mut code = String::new();

        while code.len() < 8 {
            let value = format!("{file}{n}");
            let hash = format!("{:x}", md5::compute(value));
            if hash.starts_with("00000") {
                code += format!("{}", hash.chars().nth(5).unwrap()).as_str();
            }
            n += 1;
        }
        code
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
