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

pub fn checksum(disk: Vec<Space>) -> u64 {
    disk.iter()
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

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let mut disk = parse(file);
        // until
        // first free is behind last used
        loop {
            let free_pos = disk.iter().position(|s| s == &Space::Free);
            let used_pos = disk.iter().rposition(|s| s != &Space::Free);

            if let (Some(free_idx), Some(used_idx)) = (free_pos, used_pos)
                && free_idx < used_idx
            {
                disk.swap(free_idx, used_idx);
                continue;
            }
            break;
        }

        checksum(disk)
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let disk = parse(file);


        checksum(disk)
    }
}
