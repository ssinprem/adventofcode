use std::collections::HashMap;

use regex::Regex;
// Filesystem              Size  Used  Avail  Use%
// /dev/grid/node-x0-y0     92T   73T    19T   79%
pub fn parse(file: String) -> HashMap<(usize,usize),(usize,usize,usize)> {
    let regex = Regex::new(r"\/dev\/grid\/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)\%").unwrap();
    file.lines().filter_map(|line| {
        if let Some(cap) = regex.captures(line)
        && let Some(x) = cap.get(1)
        && let Ok(x) = x.as_str().parse::<usize>()
        && let Some(y) = cap.get(2)
        && let Ok(y) = y.as_str().parse::<usize>()
        && let Some(size) = cap.get(3)
        && let Ok(size) = size.as_str().parse::<usize>()
        && let Some(used) = cap.get(4)
        && let Ok(used) = used.as_str().parse::<usize>()
        && let Some(avail) = cap.get(5)
        && let Ok(avail) = avail.as_str().parse::<usize>()
        {
            Some(((x,y),(size,used,avail)))
        } 
        else {
            None
        }
    }).collect()
}


pub mod part1 {
    use itertools::Itertools;
    use super::*;

    pub fn solve(file: String) -> usize {
        let grid = parse(file);
        let mut count = 0;
        for pair 
        in grid.into_iter().permutations(2)
        {
            let ((x,y),(_, used, _)) = pair.first().unwrap();
            let ((cx,cy),(_, _, avail)) = pair.get(1).unwrap();
            if x == cx && y == cy {
                continue;
            }
            if used != &0 &&  avail > used {
                count+=1;
                // println!("  A = {x:2},{y:2} B = {cx:2},{cy:2} {used} {avail}");
            }
        }
        count
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
