
pub mod part1 {
    use std::collections::HashMap;

    fn get_duplicate_num(md5: &mut HashMap<u64 ,(String, Vec<u64>, Vec<u64>)>, string: String, index: &u64) -> (String, Vec<u64>, Vec<u64>) {
        md5.entry(*index).or_insert_with(
                || 
                {
                    let data = format!("{}{}", string, *index);
                    let hash = format!("{:x}", md5::compute(&data));

                    let vec3 = hash.chars().collect::<Vec<char>>()
                        .windows(3).find_map(|chars| {
                            if chars[0]==chars[1] && chars[1]==chars[2] {
                                Some(
                                    i64::from_str_radix(
                                        format!("{}",chars[0]).as_str(), 16
                                    ).unwrap() as u64
                                )
                            } else {
                                None
                            }
                        });
                    let vec3 = if let Some(n) = vec3 { vec![n] } else { vec![]};
                    (
                        hash.to_string(),
                        vec3,
                        (0..=15).filter(|n| {
                            hash.contains(format!("{n:x}").repeat(5).as_str())
                        }).collect()
                    )
                }
        );
        md5.get(index).unwrap().clone()
    }

    pub fn solve(file: String) -> u64 {
        let mut list: Vec<u64> = Vec::new();
        let mut md5: HashMap<u64, (String, Vec<u64>, Vec<u64>)> = HashMap::new();
        let mut index = 1;
        while list.len() < 64 {
            let (_string, char3, _) = get_duplicate_num(&mut md5, file.to_string(), &index);
            if !char3.is_empty() 
            {
                if ((index+1)..=(index+1001)).any(|i| {
                    char3.iter().any(|n| {
                        let (_str, _, char5) = get_duplicate_num(&mut md5, file.to_string(), &i);
                        char5.contains(n)
                    })
                }) {
                    list.push(index);
                }
            }
            index += 1;
        }
        *list.last().expect("cannot get last index") as u64
    }   
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
