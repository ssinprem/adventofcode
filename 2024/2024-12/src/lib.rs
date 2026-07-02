use grid::*;

pub fn parse(file: String) -> Grid<char> {
    let cols = file.lines().find(|line| !line.is_empty()).iter().count();
    let mut map = Grid::new(0, cols);

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            map.push_row(line.chars().collect());
        });
    map
}

pub mod part1 {
    use super::*;
    const NEAR: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    fn group(map: Grid<char>) -> Vec<Vec<(isize, isize)>> {
        let cols = map.cols();
        let mut remaining: Vec<((isize, isize), char)> = map
            .iter()
            .enumerate()
            .map(|(pos, item)| (((pos / cols) as isize, (pos % cols) as isize), *item))
            .collect();

        let mut grps = Vec::new();
        while let Some(first) = remaining.pop() {
            let mut grp = Vec::<(isize,isize)>::new();
            let char = first.1;
            let mut tmp_grp = Vec::<(isize, isize)>::new();
            tmp_grp.push(first.0);
            while let Some((y,x)) = tmp_grp.pop() {
                grp.push((y,x));
                NEAR.iter().for_each(|(dy, dx)| {
                    if let Some((rid,((ry, rx), _char))) =
                        remaining.clone()
                            .iter()
                            .enumerate()
                            .find(|(_id,((ty, tx), tchar))| {
                                *tx == x + *dx && *ty == y + *dy && *tchar == char
                            })
                    {
                        remaining.remove(rid);
                        tmp_grp.push((*ry, *rx));
                    }
                });
            }
            grps.push(grp);
        }
        grps
    }

    fn get_border(group: &[(isize, isize)]) -> usize {
        let mut count = 0;
        for (x, y) in group.iter() {
            count += NEAR
                .iter()
                .filter(|(dx, dy)| !group.iter().any(|&(tx, ty)| tx == x + dx && ty == y + dy))
                .count()
        }
        count
    }

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        let groups = group(map);

        groups
            .iter()
            .map(|group| get_border(group) * group.len())
            .sum::<usize>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
