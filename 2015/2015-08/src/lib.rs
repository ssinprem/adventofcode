pub mod part1 {
    pub fn solve(file: String) -> u64 {
        file.lines().filter(|line| !line.is_empty())
        .map(|str_line| {
            let size = str_line.len();
            let line = str_line.strip_prefix('"');
            let line = line.unwrap().strip_suffix('"');
            let line = line.unwrap().chars().collect::<Vec<char>>();
            let mut len = 0;
            let mut i = 0;
            while i < size-2 {
                if line[i] != '\\' {
                    len+=1;
                    i+=1;
                } else if ['"','\\'].contains(&line[i+1]) {
                    len+=1;
                    i+=2;
                } else if line[i+1] == 'x' {
                    len+=1;
                    i+=4;
                }
            }

            size - len
        }).sum::<usize>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
