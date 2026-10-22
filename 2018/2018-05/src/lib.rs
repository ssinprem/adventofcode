use rayon::prelude::*;

pub fn process(file: String) -> String {
    let mut list = file.chars().collect::<Vec<char>>();

    loop {
        let temp = list.clone();
        if let Some(pos) = temp
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &char)>>()
            .windows(2)
            .find(|pairs| {
                let (_pos_a, char_a) = pairs[0];
                let (_pos_b, char_b) = pairs[1];

                let (a_id, a_cap) = if char_a.is_ascii_uppercase() {
                    (*char_a as u8 - b'A', true)
                } else if char_a.is_ascii_lowercase() {
                    (*char_a as u8 - b'a', false)
                } else {
                    unreachable!();
                };

                let (b_id, b_cap) = if char_b.is_ascii_uppercase() {
                    (*char_b as u8 - b'A', true)
                } else if char_b.is_ascii_lowercase() {
                    (*char_b as u8 - b'a', false)
                } else {
                    unreachable!();
                };

                a_id == b_id && a_cap != b_cap
            })
        {
            list.remove(pos[0].0);
            list.remove(pos[0].0);
        } else {
            break;
        }
    }

    list.iter().collect::<String>()
}

pub mod part1 {
    use crate::process;

    pub fn solve(file: String) -> usize {
        process(file).len()
    }
}

pub mod part2 {
    use rayon::prelude::*;
    use crate::process;

    pub fn solve(file: String) -> usize {
        let a2z = ('a'..='z').collect::<Vec<char>>();
        a2z
            .par_iter()
            .map(|remove| {
                let remove_upper = remove.to_ascii_uppercase();
                let temp = file
                    .chars()
                    .collect::<Vec<char>>()
                    .into_par_iter()
                    .filter(|char| !(char == remove || char == &remove_upper))
                    .collect::<String>();

                process(temp.to_string()).len()
            })
            .min()
            .unwrap()
    }
}
