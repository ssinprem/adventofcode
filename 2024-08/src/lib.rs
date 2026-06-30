use grid::Grid;
use std::collections::HashSet;

pub fn parse(file: String) -> (Grid<char>, HashSet<char>) {
    let cols= file.lines().find(|line| !line.is_empty()).expect("cannot get first line").len();
    let mut grid = Grid::new(0,cols);
    let mut set = HashSet::new();
    file.lines().filter(|line| !line.is_empty())
    .for_each(|line| {
        grid.push_row(line.chars().collect());
        line.chars().for_each(|char| {set.insert(char);});
    });
    (grid, set)
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let (mut grid,set) = parse(file);
        let mut count =0;
        // let mut mut_grid = grid.clone().iter().map(|char| {
        //     if ".A0".contains(*char) {
        //         char
        //     } else {
        //         &'.'
        //     }
        // }) .collect::<Grid<char>>();
        println!("{} {set:?}",set.len());
        for &target in set.iter().filter(|&c| c!=&'.') {
            let target_list: Vec<_> =grid.iter().enumerate()
                .filter(|&(_i,&char)| char == target)
                .map(|a| {
                (
                    (a.0 / grid.cols()) as isize,
                    (a.0 % grid.cols()) as isize
                )
            }).collect();
            println!("{target} -> {target_list:?}");

            for i in 0..target_list.len() {
                for j in i+1..target_list.len() {
                    let item = target_list[i];
                    let jtem = target_list[j];
                    let divy = item.0 - jtem.0;
                    let divx = item.1 - jtem.1;
                    let resonance = vec![
                        (item.0 + divy, item.1 + divx),
                        (jtem.0 - divy, jtem.1 - divx)
                    ];
                    for pos in resonance {
                        if let Some(char) = grid.get_mut(pos.0, pos.1) {
                            if char == &'.' || char == &'#' {
                            // if char == &'.' {
                                *char = '#';
                                count+=1;
                                println!("✅ {count:3}  {item:?} {jtem:?} {pos:?}");
                            }
                        }
                    }
                }
            }
        }

        for j in 0..grid.rows() {
            for i in 0..grid.cols() {
                print!("{}", grid.get(j,i).unwrap());
            }
            println!();
        }
        // grid.iter().filter(|&char| char == &'#').count() as u64
        count
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
