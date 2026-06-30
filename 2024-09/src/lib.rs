#[derive(Debug, PartialEq)]
pub enum Space {
    Free,
    Used(usize),
}

pub fn parse(file: String) -> Vec<Space> {
    let mut vec = Vec::new();

    for (i, char) in file.chars().enumerate() {
        let amount = char
            .to_string()
            .parse::<usize>()
            .expect("cannot parse size");
        (0..amount).for_each(|_| {
            if i.is_multiple_of(2) {
                // used
                vec.push(Space::Used(i / 2));
            } else {
                vec.push(Space::Free);
            }
        });
    }
    vec
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let mut vec = parse(file);
        // until
        // first free is behind last used
        loop {
            let free_pos = vec.iter().position(|s| s == &Space::Free);
            let used_pos = vec.iter().rposition(|s| s != &Space::Free);

            if let (Some(free_idx), Some(used_idx)) = (free_pos, used_pos)
                && free_idx < used_idx
            {
                vec.swap(free_idx, used_idx);
                continue;
            }
            break;
        }

        vec.iter()
            .enumerate()
            .filter_map(|(pos, space)| {
                if let Space::Used(id) = space {
                    Some(*id as u64 * pos as u64)
                } else {
                    None
                }
            })
            .sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
