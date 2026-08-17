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

    /// Calculates the next set of reachable positions in one step.
    /// This function handles the infinitely repeating grid.
    pub fn process(map: &Grid<bool>, list: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
        let rows = map.rows() as isize;
        let cols = map.cols() as isize;
        list.par_iter()
            .flat_map(|node| {
                [(-1, 0), (0, -1), (1, 0), (0, 1)].par_iter().filter_map(move |d| {
                    let new_node = (node.0 + d.0, node.1 + d.1);
                    // Wrap around the grid for infinite map
                    let mut map_r = new_node.0;
                    let mut map_c = new_node.1;
                    while map_r < 0 {
                        map_r += rows;
                    }
                    while map_r >= rows {
                        map_r -= rows;
                    }
                    while map_c < 0 {
                        map_c += cols;
                    }
                    while map_c >= cols {
                        map_c -= cols;
                    }
                    if *map.get(map_r as usize, map_c as usize).unwrap() {
                        Some(new_node)
                    } else {
                        None
                    }
                })
            })
            .collect::<HashSet<_>>()
    }

    /// Solves part 2 of the puzzle.
    /// It uses a hybrid approach:
    /// - For the specific large step count of the main puzzle, it uses a fast
    ///   quadratic extrapolation method. This method relies on properties of the
    ///   main puzzle's input file and is not a general solution.
    /// - For all other step counts (like in the tests), it uses a brute-force
    ///   simulation. This is general but would be too slow for the main puzzle.
    pub fn solve(file: String, cnt: u64) -> u64 {
        let (map, start) = parse(file);
        let mut list = HashSet::new();
        list.insert(start);
        if cnt == 26501365 {
            // Fast path for the main puzzle input using quadratic extrapolation.
            // This assumes the input grid has clear paths from the start, leading to
            // a quadratic growth in reachable plots.
            let size = map.rows() as u64;
            let rem = cnt % size;

            // We need 3 points to fit a quadratic polynomial.
            // We run the simulation for `rem`, `rem + size`, `rem + 2*size` steps.
            let mut y = vec![];
            for i in 1..=rem + 2 * size {
                list = process(&map, &list);
                if i % size == rem {
                    y.push(list.len() as u64);
                }
            }

            // Fit a quadratic polynomial P(n) = an^2 + bn + c
            // where n is the number of grid traversals.
            let n = cnt / size;
            let y0 = y[0] as i128;
            let y1 = y[1] as i128;
            let y2 = y[2] as i128;

            let a = (y2 - 2 * y1 + y0) / 2;
            let b = y1 - y0 - a;
            let c = y0;

            // Extrapolate for the given number of steps.
            (a * n as i128 * n as i128 + b * n as i128 + c) as u64
        } else {
            // General solution using brute-force simulation.
            // This is correct for any grid and step count but can be slow.
            for _ in 0..cnt {
                list = process(&map, &list);
            }
            list.len() as u64
        }
    }
}
