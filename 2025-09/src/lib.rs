#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Seat {
    x: u64,
    y: u64,
}

impl Seat {
    pub fn new_xy(x: u64,y: u64) -> Self {
        Self { x, y }
    }

    pub fn new_str(c: &[String]) -> Self {
        Self {
            x: c.first()
                .unwrap_or(&"0".to_string())
                .parse::<u64>()
                .unwrap(),
            y: c.get(1).unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
        }
    }

    pub fn new(c: Vec<String>) -> Self {
        Self {
            x: c.first().unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
            y: c.get(1).unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
        }
    }

    pub fn area(&self, b: Seat) -> u64 {
        (
            ((self.x as i64 - b.x as i64 ).abs() +1)  * 
            ((self.y as i64 - b.y as i64 ).abs() +1)
        ) as u64
    }
}


pub mod part1 {
    use std::collections::HashMap;
    use crate::Seat;

    pub fn solve(file: String) -> u64 {
        let mut areas = HashMap::<(Seat, Seat), u64>::new();
        let seats = file.lines().filter(|line| !line.is_empty())
            .map(|line| {
                let str_arr = line.split(",")
                    .map(|str| str.to_string())
                    .collect();
                Seat::new(str_arr)
            }).collect::<Vec<Seat>>();

        for i in 0..seats.len() {
            let iseat = seats.get(i).unwrap();
            for j in i+1..seats.len() {
                let jseat = seats.get(j).unwrap();
                areas.insert((*iseat,*jseat), iseat.area(*jseat));
            }
        }

        if let Some(max_entry) = areas.iter().max_by_key(|item| item.1) {
            *max_entry.1
        } else {
            0
        }
    }
}

pub mod part2 {
    use std::cmp::{ max, min};
    use crate::Seat;
    use std::collections::{HashMap, HashSet};
    pub fn solve(file: String) -> u64 {
        let red_tiles = file
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let str_arr = line
                    .split(',')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                Seat::new_str(&str_arr)
            })
            .collect::<Vec<Seat>>();

        let red_set: HashSet<Seat> = red_tiles.iter().cloned().collect();

        let mut x_coords = HashSet::new();
        let mut y_coords = HashSet::new();
        for &p in &red_tiles {
            x_coords.insert(p.x);
            y_coords.insert(p.y);
        }

        let mut sorted_x: Vec<u64> = x_coords.into_iter().collect();
        let mut sorted_y: Vec<u64> = y_coords.into_iter().collect();
        sorted_x.sort();
        sorted_y.sort();

        let x_map: HashMap<u64, usize> =
            sorted_x.iter().enumerate().map(|(i, &x)| (x, i)).collect();
        let y_map: HashMap<u64, usize> =
            sorted_y.iter().enumerate().map(|(i, &y)| (y, i)).collect();

        let mut grid = vec![vec![false; sorted_x.len()]; sorted_y.len()];

        let mut boundary = HashSet::new();
        for i in 0..red_tiles.len() {
                let p1 = red_tiles[i];
                let p2 = red_tiles[(i + 1) % red_tiles.len()];
    
                boundary.insert(p1);
    
                if p1.x == p2.x {
                    for y in min(p1.y, p2.y)..=max(p1.y, p2.y) {
                        boundary.insert(Seat { x: p1.x, y });
                    }
                } else {
                    // p1.y == p2.y
                    for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
                        boundary.insert(Seat { x, y: p1.y });
                    }
                }

        }

        for y_idx in 0..sorted_y.len() {
            let y = sorted_y[y_idx];
            let mut crossings = Vec::new();
            for i in 0..red_tiles.len() {
                let p1 = red_tiles[i];
                let p2 = red_tiles[(i + 1) % red_tiles.len()];
                if p1.x == p2.x {
                    // vertical edge
                    if (p1.y <= y && p2.y > y) || (p2.y <= y && p1.y > y) {
                        crossings.push(p1.x);
                    }
                }
            }
            crossings.sort();

            let mut is_inside = false;
            let mut cross_iter = crossings.iter().peekable();

            for x_idx in 0..sorted_x.len() {
                let x = sorted_x[x_idx];

                while let Some(&cross_x) = cross_iter.peek() {
                    if *cross_x <= x {
                        is_inside = !is_inside;
                        cross_iter.next();
                    } else {
                        break;
                    }
                }

                if is_inside || boundary.contains(&Seat { x, y }) {
                    grid[y_idx][x_idx] = true;
                }
            }
        }

        for p in red_set.iter() {
            grid[y_map[&p.y]][x_map[&p.x]] = true;
        }

        let mut max_area = 0;

        for i in 0..red_tiles.len() {
            for j in i + 1..red_tiles.len() {
                let p1 = red_tiles[i];
                let p2 = red_tiles[j];

                let x_start_idx = *x_map.get(&min(p1.x, p2.x)).unwrap();
                let x_end_idx = *x_map.get(&max(p1.x, p2.x)).unwrap();
                let y_start_idx = *y_map.get(&min(p1.y, p2.y)).unwrap();
                let y_end_idx = *y_map.get(&max(p1.y, p2.y)).unwrap();

                let mut is_valid_rect = true;
                'rect_check: for y_idx in y_start_idx..=y_end_idx {
                    for x_idx in x_start_idx..=x_end_idx {
                        if !grid[y_idx][x_idx] {
                            is_valid_rect = false;
                            break 'rect_check;
                        }
                    }
                }

                if is_valid_rect {
                    let area = p1.area(p2);
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }

        max_area
    }
}

