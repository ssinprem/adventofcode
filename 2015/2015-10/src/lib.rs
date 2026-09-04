pub mod part1 {
    pub fn look_n_say(input: String) -> String {
        if input.is_empty() {
            return "".to_string();
        }
        let mut output = "".to_string();
        let mut count = 0;
        let mut keep: Option<char> = None;
        let mut temp = input.char_indices();
        while let Some((_id, char)) = temp.next() {
            if keep.is_none() || keep.unwrap() != char {
                if count > 0 {
                    output += format!("{count}{}", keep.unwrap()).as_str();
                }
                keep = Some(char);
                count = 1;
            } else {
                count += 1;
            }
        }
        if count > 0 && keep.is_some() {
            output += format!("{count}{}", keep.unwrap()).as_str();
        }
        output
    }

    pub fn solve(file: String, n: u32) -> u64 {
        let mut str = file.to_string();
        for _i in 0..n {
            str = look_n_say(str.to_string());
        }
        str.len() as u64
    }
}

