use grid::Grid;
use std::collections::HashSet;

pub fn parse(file: String) -> (Grid<char>, HashSet<char>) {
    let cols = file
        .lines()
        .find(|line| !line.is_empty())
        .expect("cannot get first line")
        .len();
    let mut grid = Grid::new(0, cols);
    let mut set = HashSet::new();
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            grid.push_row(line.chars().collect());
            line.chars().for_each(|char| {
                set.insert(char);
            });
        });
    set.remove(&'.');
    (grid, set)
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let (mut grid, set) = parse(file);
        let mut antinode: HashSet<(isize, isize)> = HashSet::new();
        let mut count = 0;
        for target in set {
            let target_list: Vec<_> = grid
                .iter()
                .enumerate()
                .filter(|&(_i, &char)| char == target)
                .map(|a| ((a.0 / grid.cols()) as isize, (a.0 % grid.cols()) as isize))
                .collect();
            println!("{target} -> {target_list:?}");

            for i in 0..target_list.len() {
                for j in i + 1..target_list.len() {
                    let item = target_list[i];
                    let jtem = target_list[j];
                    let divy = item.0 - jtem.0;
                    let divx = item.1 - jtem.1;
                    let resonance = vec![
                        (item.0 + divy, item.1 + divx),
                        (jtem.0 - divy, jtem.1 - divx),
                    ];
                    for pos in resonance {
                        if let Some(_char) = grid.get_mut(pos.0, pos.1) {
                            if antinode.insert((pos.0, pos.1)) {
                                count += 1;
                                println!("✅ {count:3}  {pos:3?}");
                            } else {
                                println!("❕      {pos:3?} ready in antinode list");
                            }
                        } else {
                            println!("❕      {pos:3?} is outside map");
                        }
                    }
                }
            }
        }
        count // antinode.len() as u64
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let (mut grid, set) = parse(file);
        let mut antinode: HashSet<(isize, isize)> = HashSet::new();
        let mut count = 0;
        for target in set {
            let target_list: Vec<_> = grid
                .iter()
                .enumerate()
                .filter(|&(_i, &char)| char == target)
                .map(|a| ((a.0 / grid.cols()) as isize, (a.0 % grid.cols()) as isize))
                .collect();
            println!("{target} -> {target_list:?}");

            for i in 0..target_list.len() {
                for j in i + 1..target_list.len() {
                    let item = target_list[i];
                    let jtem = target_list[j];
                    let divy = item.0 - jtem.0;
                    let divx = item.1 - jtem.1;
                    let resonances = vec![(item, (divy, divx)), (jtem, (-divy, -divx))];
                    for (ant, div) in resonances {
                        let mut mul = 0;
                        loop {
                            let pos = (ant.0 + mul * div.0, ant.1 + mul * div.1);
                            if let Some(_char) = grid.get_mut(pos.0, pos.1) {
                                if antinode.insert((pos.0, pos.1)) {
                                    count += 1;
                                    println!("✅ {count:3}  {pos:3?}");
                                } else {
                                    println!("❕      {pos:3?} ready in antinode list");
                                }
                            } else {
                                println!("❕      {pos:3?} is outside map");
                                break;
                            }
                            mul += 1;
                        }
                    }
                }
            }
        }
        count // antinode.len() as u64
    }
}
