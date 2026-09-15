pub mod part1 {

    use regex::Regex;

    pub fn decompress(file: String) -> String {
        let mut input = file.to_string();
        let mut output = String::new();

        let regex = Regex::new(r"(?:^\(([0-9]+)x([0-9]+)\))|(^[A-Z]+)").unwrap();
        while !input.is_empty() {
            if let Some(caps) = regex.captures(&input) {
                println!("{:?}", caps);
                if let Some(size_txt) = caps.get(1)
                    && let Some(times_txt) = caps.get(2)
                    && let Ok(size) = size_txt.as_str().parse::<u32>()
                    && let Ok(times) = times_txt.as_str().parse::<u32>()
                {
                    input = input.split_off(3 + size_txt.len() + times_txt.len());
                    let temp = input.split_off(size as usize);

                    output += input.repeat(times as usize).as_str();
                    input = temp;
                } else if let Some(str) = caps.get(3) {
                    output += str.as_str();
                    input = input.split_off(str.len());
                } else {
                    unreachable!()
                }
            }
        }

        output
    }

    pub fn solve(file: String) -> u64 {
        decompress(file).len() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
