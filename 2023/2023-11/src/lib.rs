use grid::*;

pub fn parse(file: String) -> Grid<bool> {
    let mut maps: Grid<bool> = Grid::new(0, 0);
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            maps.push_row(line.chars().map(|char| matches!(char, '#')).collect());
        });
    maps
}

pub fn expand_maps(maps: &mut Grid<bool>) {
    let empty_rows: Vec<usize> = maps
        .iter_rows()
        .enumerate()
        .filter_map(|(n, mut row)| if row.all(|c| !c) { Some(n) } else { None })
        .collect();
    let empty_cols: Vec<usize> = maps
        .iter_cols()
        .enumerate()
        .filter_map(|(n, mut col)| if col.all(|c| !c) { Some(n) } else { None })
        .collect();

    for (i, r) in empty_rows.iter().enumerate() {
        maps.insert_row(r + i, vec![false; maps.cols()]);
    }
    for (i, c) in empty_cols.iter().enumerate() {
        maps.insert_col(c + i, vec![false; maps.rows()]);
    }
}

pub fn _display(maps: &Grid<bool>, list: &[(isize, isize)]) {
    println!();
    maps.iter_rows().enumerate().for_each(|(y, row)| {
        for (x, cell) in row.enumerate() {
            if !list.contains(&(y as isize, x as isize)) {
                print!(
                    "{}",
                    match *cell {
                        true => "#",
                        false => ".",
                    }
                );
            } else {
                print!(
                    "{}",
                    match *cell {
                        true => "◘",
                        false => "•",
                    }
                );
            }
        }
        println!();
    });
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let mut maps = parse(file);
        _display(&maps, &[]);
        expand_maps(&mut maps);
        _display(&maps, &[]);

        let width = maps.cols();
        let nodes: Vec<_> = maps
            .iter()
            .enumerate()
            .filter_map(|(n, value)| {
                let y = n / width;
                let x = n % width;
                if *value { Some((x, y)) } else { None }
            })
            .collect();

        let pairs: Vec<_> = nodes
            .iter()
            .enumerate()
            .flat_map(|(i, a)| nodes[i + 1..].iter().map(move |b| (a, b)))
            .collect();

        pairs
            .iter()
            .map(|((ax, ay), (bx, by))| ax.abs_diff(*bx) + ay.abs_diff(*by))
            .sum::<usize>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
