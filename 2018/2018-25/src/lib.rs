pub fn parse(file: String) -> Vec<[isize; 4]> {
    let regex = regex::Regex::new(r"([-0-9]+),([-0-9]+),([-0-9]+),([-0-9]+)").unwrap();
    file.lines()
        .map(|line| {
            let mut set = [0; 4];
            let caps = regex.captures(line).unwrap();
            for (i, n) in set.iter_mut().enumerate() {
                *n = caps.get(1 + i).unwrap().as_str().parse::<isize>().unwrap();
            }
            set
        })
        .collect()
}

pub fn pts_range(a: &[isize; 4], b: &[isize; 4]) -> usize {
    (0..4).map(|i| a[i].abs_diff(b[i])).sum()
}

pub fn group_consetellations(pts: &[[isize; 4]], range: usize) -> Vec<Vec<[isize; 4]>> {
    let mut pts = pts.to_vec();
    let mut grps = Vec::new();

    while !pts.is_empty() {
        let mut grp = vec![];
        grp.push(pts.pop().unwrap());
        while let Some(index) = pts
            .iter()
            .position(|p| grp.iter().any(|g| pts_range(p, g) <= range))
        {
            grp.push(pts.remove(index));
        }
        println!("{grp:?}");
        grps.push(grp);
    }

    grps
}

pub mod part1 {
    use crate::{group_consetellations, parse};

    pub fn solve(file: String) -> usize {
        let pts = parse(file);

        group_consetellations(&pts, 3).len()
    }
}
