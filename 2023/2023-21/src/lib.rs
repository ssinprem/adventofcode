use grid::*;
use rayon::prelude::*;

pub fn parse(file: String) -> (Grid<bool>, (isize, isize)) {
    let mut map = Grid::new(0, 0);
    let mut start = (0_isize, 0_isize);
    file.lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(row, line)| {
            if line.contains("S") {
                start = (
                    row as isize,
                    line.chars().position(|c| c == 'S').unwrap() as isize
                )
            }
            map.push_row(line.chars().map(|c| c != '#').collect());
        });
    (map, start)
}



pub fn _display(map: &Grid<bool>, list: &[(isize, isize)]) -> String {
    let mut string = "".to_string();

    let rows = map.rows();
    let cols = map.cols();

    for row in 0..rows {
        for col in 0..cols {
            if list.contains(&(row as isize, col as isize)) {
                string += "O";
            } else if let Some(c) = map.get(row, col)
                && *c
            {
                string += ".";
            } else {
                string += "#";
            }
        }
        string += "\n";
    }

    string
}

pub mod part1 {
    use super::*;

    pub fn process(map: &Grid<bool>, list: &[(isize, isize)]) -> Vec<(isize, isize)> {
        let mut new_list = Vec::new();

        for node in list {
            for d in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let new_node = (node.0 + d.0, node.1 + d.1);
                if let Some(is_path) = map.get(new_node.0, new_node.1)
                    && *is_path
                {
                    let new_node = (new_node.0 as isize, new_node.1 as isize);
                    if !new_list.contains(&new_node) {
                        new_list.push(new_node);
                    }
                }
            }
        }

        new_list
    }

    pub fn solve(file: String, cnt: u64) -> u64 {
        let (map, start) = parse(file);
        let mut list = vec![start];
        println!("{}", _display(&map, &list));

        for _i in 0..cnt {
            list = process(&map, &list);
        }
        println!("{}", _display(&map, &list));

        list.len() as u64
    }
}

pub mod part2 {
    use super::*;
    use std::collections::HashSet;

    pub fn process(map: &Grid<bool>, list: &[(isize, isize)]) -> Vec<(isize, isize)> {
        let rows = map.rows() as isize;
        let cols = map.cols() as isize;
        list
            .par_iter()
            .flat_map(|node| {
                let mut nodes = Vec::new();
                for d in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    let new_node = (node.0 + d.0, node.1 + d.1);
                    let mut map_r = new_node.0;
                    let mut map_c = new_node.1;
                    while map_r < 0 {
                        map_r += rows
                    }
                    while map_r >= rows {
                        map_r -= rows
                    }
                    while map_c < 0 {
                        map_c += cols
                    }
                    while map_c >= cols {
                        map_c -= cols
                    }
                    if let Some(is_path) = map.get(map_r, map_c)
                        && *is_path
                    {
                        nodes.push((new_node.0 as isize, new_node.1 as isize));
                    }
                }
                nodes
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<(isize,isize)>>()
    }

    pub fn solve(file: String, cnt: u64) -> u64 {
        let (map, start) = parse(file);
        let mut list = vec![start];
        for _i in 0..cnt {
            list = process(&map, &list);
            println!("{_i} {}", list.len());
        }
        list.len() as u64
    }
}
