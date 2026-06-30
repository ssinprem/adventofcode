#[derive(Debug, PartialEq, Clone, Copy)]
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

pub fn print_disk(disk: Vec<Space>) {
    for space in disk {
        if space == Space::Free {
            print!(".");
        } else if let Space::Used(c) = space {
            print!("{}", c);
        }
    }
    println!();
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
        let mut disk = parse(file.clone());
        let max_chunk = file.len();
        let mut skip = 0;
        loop {
            // find free space chuck
            let binding = disk
                .iter()
                .enumerate()
                .map(|(i, s)| (i, *s))
                .collect::<Vec<(usize, Space)>>();
            let chunks: Vec<_> = binding.chunk_by(|a, b| a.1 == b.1).collect();

            if 
                let Some(&useds) = chunks.iter().rev().skip(skip).find(
                    |&&vec| vec.iter().all(|s| s.1!=Space::Free)) 
               &&
                let Some(&frees) = chunks.iter().find(|&&vec| 
                    vec.iter().all(|s| s.1 == Space::Free
                    && useds.len() <= vec.len()
                ))
               &&
               frees[0].0 < useds[0].0
            {
                let free_pos = frees.iter().map(|s| s.0);
                let used_pos = useds.iter().map(|s| s.0);
                for (i, j) in free_pos.zip(used_pos) {
                    disk.swap(i, j);
                }
                continue;
            } else if skip > max_chunk / 2 {
                break;
            }
            skip+=1;
        }
        checksum(disk)
    }
}
