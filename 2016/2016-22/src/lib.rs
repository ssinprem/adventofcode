use std::collections::HashMap;
use itertools::Itertools;
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

pub fn get_pairs_available(grid: &HashMap<(usize, usize),(usize,usize,usize)>, adjustment: bool)
 -> Vec<((usize,usize),(usize,usize))>
{
    let mut list = Vec::new();
    for pair 
        in grid.into_iter().permutations(2)
        {
            let ((x,y),(_, used, _)) = pair.first().unwrap();
            let ((cx,cy),(_, _, avail)) = pair.get(1).unwrap();
            if adjustment {
                if *cx < *x-1 || *cx > *x+1 {
                    continue;
                }
                if *cy < *y-1 || *cy > *y+1 {
                    continue;
                }
            }
            if x == cx && y == cy {
                continue;
            }
            if used != &0 &&  avail > used {
                list.push(((*x,*y),(*cx,*cy)));
                // println!("  A = {x:2},{y:2} B = {cx:2},{cy:2} {used} {avail}");
            }
        }
    list
}
pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> usize {
        let grid = parse(file);
        get_pairs_available(&grid, false).len()
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> usize {
        let grid = parse(file);
        let mut stack = vec![(grid, 0)];
        let mut best = usize::MAX;
        
        while let Some((map, count)) = stack.pop() {

            for (src, dst ) in get_pairs_available(&map, true) {
                let mut new_map = map.clone();
                let src_used = new_map.get(&(src.0,src.1)).unwrap().1;
                let src_size = new_map.get(&(src.0,src.1)).unwrap().0;
                {
                    let dst_cell = new_map.get_mut(&(dst.0,dst.1)).unwrap();
                    dst_cell.2 -= src_used;
                    dst_cell.1 += src_used;
                }
                {
                    let src_cell = new_map.get_mut(&(src.0,src.1)).unwrap();
                    src_cell.2 = src_size;
                    src_cell.1 = 0;
                }
                stack.push((new_map, count+1));
            }
        }

        best
    }
}
