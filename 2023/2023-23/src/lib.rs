use grid::*;
use rayon::prelude::*;

pub fn parse(file: String) -> (Grid<char>, (usize, usize), (usize, usize)) {
    let mut maps: Grid<char> = Grid::new(0, 0);

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            maps.push_row(line.chars().collect());
        });

    let rows = maps.rows();
    let cols = maps.cols();
    let start = (0, 1);
    let end = (rows - 1, cols - 2);

    (maps, start, end)
}

pub mod part1 {
    use super::*;

    pub fn pathing(
        maps: &Grid<char>,
        path: Vec<(usize, usize)>,
        end: (usize, usize),
    ) -> Vec<Vec<(usize, usize)>> {
        let rows = maps.rows();
        let cols = maps.cols();
        let mut path = path.clone();
        while let Some(cur) = path.last()
            && cur != &end
        {
            let candidate = [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .iter()
                .filter_map(|(dy, dx)| {
                    let new_y = cur.0 as isize + dy;
                    let new_x = cur.1 as isize + dx;

                    if new_y < 0
                        || new_y >= rows as isize
                        || new_x < 0
                        || new_x >= cols as isize
                        || path.contains(&(new_y as usize, new_x as usize))
                    {
                        None
                    } else if let Some(block) = maps.get(new_y, new_x) {
                        if block == &'.'
                            || (block == &'^' && (dy, dx) == (&-1, &0))
                            || (block == &'v' && (dy, dx) == (&1, &0))
                            || (block == &'<' && (dy, dx) == (&0, &-1))
                            || (block == &'>' && (dy, dx) == (&0, &1))
                        {
                            Some((new_y as usize, new_x as usize))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if candidate.len() == 1 {
                path.push(*candidate.last().unwrap());
            } else if candidate.len() > 1 {
                return candidate
                    .par_iter()
                    .flat_map(|can| {
                        let mut path = path.clone();
                        path.push(*can);
                        pathing(maps, path, end)
                    })
                    .collect();
            } else {
                return vec![];
            }
        }
        vec![path]
    }

    pub fn solve(file: String) -> u64 {
        let (maps, start, end) = parse(file);
        let paths = pathing(&maps, vec![start], end);
        paths
            .iter()
            // .inspect(|p| println!("{}", p.len()))
            .max_by_key(|p| p.len())
            .unwrap()
            .len() as u64
            - 1
    }
}

pub mod part2 {
    use std::collections::{HashMap, HashSet};
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (maps, start, end) = parse(file);
        let rows = maps.rows();
        let cols = maps.cols();

        let mut nodes = vec![start, end];
        for r in 0..rows {
            for c in 0..cols {
                if let Some(&ch) = maps.get(r, c) {
                    if ch != '#' {
                        let mut neighbors = 0;
                        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let nr = r as isize + dr;
                            let nc = c as isize + dc;
                            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                                if let Some(&n_ch) = maps.get(nr as usize, nc as usize) {
                                    if n_ch != '#' {
                                        neighbors += 1;
                                    }
                                }
                            }
                        }
                        if neighbors > 2 && (r, c) != start && (r, c) != end {
                            nodes.push((r, c));
                        }
                    }
                }
            }
        }

        let node_indices: HashMap<(usize, usize), usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, i))
            .collect();

        let mut adj = vec![vec![]; nodes.len()];

        for (i, &p) in nodes.iter().enumerate() {
            let mut stack = vec![(p, 0u64)];
            let mut visited = HashSet::new();
            visited.insert(p);

            while let Some((cur, dist)) = stack.pop() {
                if dist > 0 && node_indices.contains_key(&cur) {
                    let target_idx = node_indices[&cur];
                    adj[i].push((target_idx, dist));
                    continue;
                }

                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nr = cur.0 as isize + dr;
                    let nc = cur.1 as isize + dc;
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        let next_p = (nr as usize, nc as usize);
                        if let Some(&n_ch) = maps.get(next_p.0, next_p.1) {
                            if n_ch != '#' && !visited.contains(&next_p) {
                                visited.insert(next_p);
                                stack.push((next_p, dist + 1));
                            }
                        }
                    }
                }
            }
        }

        fn dfs(
            node: usize,
            end_node: usize,
            visited: u64,
            adj: &Vec<Vec<(usize, u64)>>,
        ) -> Option<u64> {
            if node == end_node {
                return Some(0);
            }

            let mut max_dist = None;
            for &(next_node, dist) in &adj[node] {
                if (visited & (1 << next_node)) == 0 {
                    if let Some(sub_dist) = dfs(next_node, end_node, visited | (1 << next_node), adj) {
                        let total = dist + sub_dist;
                        max_dist = Some(max_dist.map_or(total, |m: u64| m.max(total)));
                    }
                }
            }
            max_dist
        }

        let start_node = node_indices[&start];
        let end_node = node_indices[&end];

        dfs(start_node, end_node, 1 << start_node, &adj).unwrap_or(0)
    }
}
