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
                let dist = (*cx as isize - *x as isize).abs() + (*cy as isize - *y as isize).abs();
                if dist != 1 {
                    continue;
                }
            }
            if x == cx && y == cy {
                continue;
            }
            if used != &0 && avail > used {
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
    use std::collections::VecDeque;

    pub fn solve(file: String) -> usize {
        let grid = parse(file);
        let max_x = grid.keys().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = grid.keys().map(|(_, y)| *y).max().unwrap_or(0);
        let width = max_x + 1;
        let height = max_y + 1;
        let num_nodes = width * height;

        let (empty_pos, empty_size) = grid
            .iter()
            .find(|(_, (_, used, _))| *used == 0)
            .map(|(&pos, &(size, _, _))| (pos, size))
            .expect("empty node not found");

        let goal_pos = (max_x, 0);

        let is_wall = |pos: (usize, usize)| -> bool {
            if let Some(&(_, used, _)) = grid.get(&pos) {
                used > empty_size
            } else {
                true
            }
        };

        let to_idx = |pos: (usize, usize)| pos.1 * width + pos.0;
        let start_empty_idx = to_idx(empty_pos);
        let start_goal_idx = to_idx(goal_pos);
        let target_goal_idx = to_idx((0, 0));

        if start_goal_idx == target_goal_idx {
            return 0;
        }

        let mut visited = vec![false; num_nodes * num_nodes];
        let mut queue = VecDeque::new();

        visited[start_empty_idx * num_nodes + start_goal_idx] = true;
        queue.push_back((0, empty_pos, goal_pos));

        while let Some((dist, (ex, ey), (gx, gy))) = queue.pop_front() {
            let moves = [
                (ex.wrapping_sub(1), ey, ex > 0),
                (ex + 1, ey, ex + 1 <= max_x),
                (ex, ey.wrapping_sub(1), ey > 0),
                (ex, ey + 1, ey + 1 <= max_y),
            ];

            for (nex, ney, valid) in moves {
                if !valid {
                    continue;
                }
                let next_empty = (nex, ney);
                if is_wall(next_empty) {
                    continue;
                }
                let next_goal = if next_empty == (gx, gy) {
                    (ex, ey)
                } else {
                    (gx, gy)
                };

                let next_empty_idx = to_idx(next_empty);
                let next_goal_idx = to_idx(next_goal);

                if next_goal_idx == target_goal_idx {
                    return dist + 1;
                }

                let state_idx = next_empty_idx * num_nodes + next_goal_idx;
                if !visited[state_idx] {
                    visited[state_idx] = true;
                    queue.push_back((dist + 1, next_empty, next_goal));
                }
            }
        }

        0
    }
}
