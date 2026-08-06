use std::{
    cmp::Reverse, collections::{BinaryHeap, HashMap}, println
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
    start_pos: (isize, isize)
) -> HashMap<(isize, isize), (u64, Vec<(isize,isize)>)> {
    let mut dists = HashMap::new();
    let mut pq: BinaryHeap<(Reverse<u64>, (isize, isize), Vec<(isize,isize)>)>  = BinaryHeap::new();
    dists.insert(start_pos, (0, vec![]));
    pq.push((Reverse(0), start_pos, vec![]));
    while let Some((Reverse(score), pos, pre_jumps)) = pq.pop() {
        // println!("{pos:?} {score}");
        let mut path = [].to_vec();
        if dists.get(&pos).is_none() {

        } else {
            let (old_score, npath) = dists.get(&pos).unwrap();
            path = npath.clone();
            if score > *old_score {
                continue;
            }
        }
        for jump in [(0,1), (1,0), (-1,0), (0, -1)] {
            let next_pos = (pos.0 + jump.0, pos.1 + jump.1);
            print!("{next_pos:2?} ");
            let mut nodes = pre_jumps.clone();
            let last = pre_jumps.len();
            if last >=1 && pre_jumps[last-1] == (-jump.0, -jump.1) {
                println!("ignore backward");
                continue;
            }
            if last >= 4
                && pre_jumps[last-1] == jump
                && pre_jumps[last-2] == jump
                && pre_jumps[last-3] == jump
                && pre_jumps[last-4] == jump
            {
                println!("ignore over 3 straight");
                continue;
            }
            
            if let Some(cell_score) = map.get(next_pos.0, next_pos.1) {
                let new_score = score + *cell_score as u64;
                if let Some((current_best, _current_path)) = dists.get(&next_pos) {
                    if new_score < *current_best {
                        let mut new_path = path.clone();
                        new_path.push(next_pos);
                        nodes.push(jump);
                        println!("new score {new_score} {new_path:?}");
                        dists.insert(next_pos, (new_score, new_path));
                        pq.push((Reverse(new_score), next_pos, nodes));
                    } else {
                        println!("ignore bad score {new_score} > current_best");
                    }
                } else {
                    let mut new_path = path.clone();
                    new_path.push(next_pos);
                    nodes.push(jump);
                    println!("first meet {new_score} {new_path:?}");
                    dists.insert(next_pos, (new_score, new_path));
                    pq.push((Reverse(new_score), next_pos, nodes));
                }
            } else {
                println!("ignore unknown position");
            }
        }
        for row in 0..map.rows() {
            for col in 0..map.cols() {
                if let Some((s,_)) = dists.get(&(row as isize, col as isize)) {
                    print!("{:3} ", s);
                } else {
                    print!("___ ");
                }
            }
            println!();
        }
    }
    dists
}

pub mod part1 {

use super::*;
    pub fn solve(file: String) -> u64 {
        let map = parse(file);

        let dists = dijkstra(&map, (0,0));
        let end = (map.rows() as isize -1, map.cols() as isize -1);

        println!("{}", _display(&map));
        
        let last_dist = dists.get(&end).unwrap();
        for row in 0..map.rows() {
            for col in 0..map.cols() {
                if last_dist.1.contains(&(row as isize,col as isize)) {
                    print!("{:3} ", dists.get(&(row as isize, col as isize)).unwrap().0);
                } else {
                    print!("    ");
                }
            }
            println!();
        }

        last_dist.0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
