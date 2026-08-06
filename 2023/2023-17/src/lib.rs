use std::{
    cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, println
};

use grid::*;

pub fn parse(file: String) -> Grid<u8> {
    let mut map = Grid::new(0,0);
    file.lines().filter(|line| !line.is_empty())
    .for_each(|line| {
        map.push_row(
            line.chars().filter_map(|c| c.to_digit(10).map(|d| d as u8)).collect()
        );
    });
    map
}

pub fn _display(map: &Grid<u8>) -> String {
    let mut string = String::new();

    let cols = map.cols();
    let rows = map.rows();

    for row in 0..rows {
        for col in 0..cols {
            let n = map.get(row, col).unwrap();
            string += n.to_string().as_str();
        }
        string += "\n"
    }
    string
}

fn _pause() {
    use std::io;
    use std::io::prelude::*;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn dijkstra(
    map: &Grid<u8>,
    start_pos: (isize, isize),
    end_pos: (isize, isize),
    _min_step: usize,
    max_step: usize
) -> (u64, Vec<(isize,isize)>) {

    
    let mut pq: BinaryHeap<(Reverse<u64>, (isize, isize), (isize,isize), usize, Vec<(isize,isize)>)>  = BinaryHeap::new();
    let mut visited = HashSet::new();
    
    pq.push((Reverse(0), start_pos, (0,0), 0, vec![start_pos]));
    while let Some((Reverse(heat_loss), pos, dir, steps, path)) = pq.pop() {

        if pos == end_pos{
            return (heat_loss, path);
        }

        if !visited.insert((pos,dir,steps)) {
            continue;
        }
        for jump in [(0,1), (1,0), (-1,0), (0, -1)] {
            let next_pos = (pos.0 + jump.0, pos.1 + jump.1);
            // print!("{next_pos:2?} ");

            if dir == jump && steps+1 >= max_step {
                // println!("ignore over straight");
                continue;
            }

            if dir == (-jump.0, -jump.1) {
                // println!("ignore backward");
                continue;
            }

            let new_steps = if dir == jump {
                steps + 1
            } else {
                1
            };

            if let Some(heat) = map.get(next_pos.0, next_pos.1) {

                if path.contains(&next_pos) {
                    // println!("ignore visited path");
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(next_pos);
                let new_loss = heat_loss + *heat as u64;
                // println!("added {new_loss}");
                if ! visited.contains(&(next_pos, jump, new_steps)) {
                    // for row in 0..map.rows() {
                    //     for col in 0..map.cols() {
                    //         if let Some(order) = new_path.iter().position(|&(arow,acol)| arow==row as isize && acol==col as isize) {
                    //             print!("{:3} ", order);
                    //         } else {
                    //             print!("___ ");
                    //         }
                    //     }
                    //     println!();
                    // }
                    pq.push((Reverse(new_loss), next_pos, jump, new_steps, new_path));
                }
            } else {
                // println!("outsite map");
                continue;
            }
        }
    }
    (0, vec![])
}

pub mod part1 {

use super::*;
    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        
        println!("{}", _display(&map));
        let (heat, path) = dijkstra(&map, (0,0), 
        (map.rows() as isize -1, map.cols() as isize -1),
         1, 4
        );

        let mut sum: u64 = 0;
        let heat_path : Vec<_>= (0..path.len()).map(|n| {
            let pos = path[n];
            let h = map.get(pos.0, pos.1).unwrap();
            if n > 0 {
                sum += *h as u64;
            }
            (pos, sum)
        }).collect();
        for row in 0..map.rows() {
            for col in 0..map.cols() {
                if let Some(order) = heat_path.iter().position(|&((arow,acol),_)| arow==row as isize && acol==col as isize)
                {
                    print!("{:3} ", heat_path[order].1);
                } else {
                    print!("___ ");
                }
            }
            println!();
        }
        heat
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
