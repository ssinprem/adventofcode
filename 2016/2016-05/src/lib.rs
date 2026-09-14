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
    pub fn solve(file: String) -> String {
        let mut n = 1;
        let mut code = "________".to_string();

        while code.contains("_") {
            let value = format!("{file}{n}");
            let hash = format!("{:x}", md5::compute(value));
            if hash.starts_with("00000") 
            && let Some(pos) = hash.chars().nth(5)
            && let Ok(pos) = format!("{pos}").as_str().parse::<usize>()
            && pos <= 7
            && code.chars().nth(pos) == Some('_')
            && let Some(char) = hash.chars().nth(6)
            {
                code.replace_range(
                    pos..pos+1, 
                    format!("{char}").as_str()
                );
                println!("{code}");
            }
            n += 1;
        }
        code
    }
}
