pub mod part1 {
    use regex::Regex;
    pub fn parse(file: String) -> Vec<u32> {
        let width = file.lines().find(|line| !line.is_empty()).unwrap().len() + 1;
        let regex_num = Regex::new(r"(\d+)").unwrap();
        let regex_symb = Regex::new(r"([^0-9.\n])").unwrap();
        let symbols: Vec<_> = regex_symb.captures_iter(&file).collect();
        regex_num
            .captures_iter(&file)
            .filter_map(|cap| {
                let found = cap.get(1).unwrap().range().any(|pos| {
                    let nx = (pos % width) as isize;
                    let ny = (pos / width) as isize;
                    symbols.iter().any(|symb| {
                        let sym_pos = symb.get(1).unwrap().start();
                        let sx = (sym_pos % width) as isize;
                        let sy = (sym_pos / width) as isize;
                        (-1..=1).any(|dy| (-1..=1).any(|dx| nx + dx == sx && ny + dy == sy))
                    })
                });

                if found {
                    Some(cap.get(1).unwrap().as_str().parse::<u32>().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn solve(file: String) -> u64 {
        let set = parse(file);
        set.iter().sum::<u32>() as u64
    }
}

pub mod part2 {
    use regex::Regex;
    pub fn parse(file: String) -> Vec<u32> {
        let width = file.lines().find(|line| !line.is_empty()).unwrap().len() + 1;
        let regex_num = Regex::new(r"(\d+)").unwrap();
        let regex_symb = Regex::new(r"([^0-9.\n])").unwrap();
        let symbols: Vec<_> = regex_symb.captures_iter(&file).collect();
        let nums: Vec<_> = regex_num.captures_iter(&file).collect();

        symbols
            .iter()
            .filter_map(|sym_cap| {
                let sym_pos = sym_cap.get(1).unwrap().start();
                let sx = (sym_pos % width) as isize;
                let sy = (sym_pos / width) as isize;

                let near_nums = nums.iter().filter_map(|num_cap| {
                    let is_near = num_cap.get(1).unwrap().range().any(|pos| {
                        let nx = (pos % width) as isize;
                        let ny = (pos / width) as isize;
                        (-1..=1).any(|dy| (-1..=1).any(|dx| nx + dx == sx && ny + dy == sy))
                    });

                    if is_near {
                        Some(num_cap.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    } else {
                        None
                    }
                });

                if near_nums.clone().count() >= 2 {
                    Some(near_nums.product::<u32>())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn solve(file: String) -> u64 {
        let set = parse(file);
        set.iter().sum::<u32>() as u64
    }
}
