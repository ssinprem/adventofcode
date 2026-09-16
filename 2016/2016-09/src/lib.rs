pub mod part1 {

    use regex::Regex;

    pub fn decompress(file: String) -> String {
        let mut input = file.to_string();
        let mut output = String::new();

        let regex = Regex::new(r"(?:^\(([0-9]+)x([0-9]+)\))|(^[A-Z]+)").unwrap();
        while !input.is_empty() {
            if let Some(caps) = regex.captures(&input) {
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
    use regex::Regex;

    pub fn decompress(file: String) -> String {
        let mut string = file.to_string();

        let regex = Regex::new(r"\(([0-9]+)x([0-9]+)\)(.*)").unwrap();
        while let Some(caps)  = regex.captures(&string) {
            println!("{} {}", string.len(), string.chars().into_iter().filter(|c| *c=='(').count());
            if let Some(size_txt) = caps.get(1)
                && let Some(times_txt) = caps.get(2)
                && let Ok(size) = size_txt.as_str().parse::<usize>()
                && let Ok(times) = times_txt.as_str().parse::<usize>()
                && let Some(text) = caps.get(3)
                && text.len() >= size
            {
                let (front, _) = string.split_once("(").expect("cannot find '('");
                let (_, output) = string.split_once(")").expect("cannot find ')'");
                let mut output = output.to_string();
                let back = output.split_off(size);
                output = output.repeat(times);
                string = front.to_string() + output.as_str() + back.as_str();
            } else {
                unreachable!()
            }
        }
        // println!("{string}");
        string
    }

    pub fn solve(file: String) -> u64 {
        decompress(file).len() as u64
    }
}
