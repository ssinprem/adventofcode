use regex::Regex;
use std::collections::HashMap;

use building_blocks_core::{Point3i, PointN};

pub fn parse(file: String) -> HashMap<u32, Vec<Point3i>> {
    let regex = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();

    file.lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(n, line)| {
            let capture = regex.captures(line).unwrap();
            let x1 = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y1 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let z1 = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let x2 = capture.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let y2 = capture.get(5).unwrap().as_str().parse::<i32>().unwrap();
            let z2 = capture.get(6).unwrap().as_str().parse::<i32>().unwrap();

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            let min_z = z1.min(z2);
            let max_z = z1.max(z2);

            let mut brick = Vec::new();
            for z in min_z..=max_z {
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        brick.push(PointN([x, y, z]));
                    }
                }
            }
            (n as u32, brick)
        })
        .collect()
}

pub mod part1 {
    use super::*;
    use itertools::Itertools;
    use std::collections::HashSet;

    pub fn solve(file: String) -> u64 {
        let mut bricks: Vec<_> = parse(file).into_iter().collect();
        bricks.sort_by_key(|(_, b)| b.iter().map(|p| p.z()).min().unwrap());

        let mut space: HashMap<(i32, i32), (i32, u32)> = HashMap::new();
        let mut supports: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut supported_by: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (id, brick) in &mut bricks {
            let mut max_z = 0;
            let mut support_candidates = HashSet::new();

            let bottom_points: Vec<_> = brick
                .iter()
                .map(|p| (p.x(), p.y()))
                .unique()
                .collect();

            for (x, y) in bottom_points {
                if let Some((z, support_id)) = space.get(&(x, y)) {
                    if *z > max_z {
                        max_z = *z;
                        support_candidates.clear();
                        support_candidates.insert(*support_id);
                    } else if *z == max_z && max_z != 0 {
                        support_candidates.insert(*support_id);
                    }
                }
            }

            // Record support relationships
            if let Some(s) = supported_by.get_mut(id) {
                for v in support_candidates.iter() {
                    s.insert(*v);
                }
            } else {
                supported_by.insert(*id, support_candidates.clone());
            }

            for supporter_id in support_candidates {
                supports.entry(supporter_id).or_default().insert(*id);
            }

            // Move brick down
            let min_z_in_brick = brick.iter().map(|p| p.z()).min().unwrap();
            let fall_dist = min_z_in_brick - max_z - 1;
            if fall_dist > 0 {
                for p in brick.iter_mut() {
                    *p.z_mut() -= fall_dist;
                }
            }

            // Update space
            for p in brick.iter() {
                space.insert((p.x(), p.y()), (p.z(), *id));
            }
        }

        // Now solve part 1
        let count = bricks
            .iter()
            .filter(|(id, _)| {
                if let Some(supported_bricks) = supports.get(id) {
                    supported_bricks.iter().all(|supported_id| {
                        supported_by
                            .get(supported_id)
                            .map_or(false, |s| s.len() > 1)
                    })
                } else {
                    // No bricks supported by this one, so it can be disintegrated.
                    true
                }
            })
            .count();

        count as u64
    }
}

pub mod part2 {
    use super::*;
    use itertools::Itertools;
    use std::collections::HashSet;

    pub fn solve(file: String) -> u64 {
        let mut bricks: Vec<_> = parse(file).into_iter().collect();
        bricks.sort_by_key(|(_, b)| b.iter().map(|p| p.z()).min().unwrap());

        let mut space: HashMap<(i32, i32), (i32, u32)> = HashMap::new();
        let mut supports: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut supported_by: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (id, brick) in &mut bricks {
            let mut max_z = 0;
            let mut support_candidates = HashSet::new();

            let bottom_points: Vec<_> = brick
                .iter()
                .map(|p| (p.x(), p.y()))
                .unique()
                .collect();

            for (x, y) in bottom_points {
                if let Some((z, support_id)) = space.get(&(x, y)) {
                    if *z > max_z {
                        max_z = *z;
                        support_candidates.clear();
                        support_candidates.insert(*support_id);
                    } else if *z == max_z && max_z != 0 {
                        support_candidates.insert(*support_id);
                    }
                }
            }

            // Record support relationships
            if let Some(s) = supported_by.get_mut(id) {
                for v in support_candidates.iter() {
                    s.insert(*v);
                }
            } else {
                supported_by.insert(*id, support_candidates.clone());
            }

            for supporter_id in support_candidates {
                supports.entry(supporter_id).or_default().insert(*id);
            }

            // Move brick down
            let min_z_in_brick = brick.iter().map(|p| p.z()).min().unwrap();
            let fall_dist = min_z_in_brick - max_z - 1;
            if fall_dist > 0 {
                for p in brick.iter_mut() {
                    *p.z_mut() -= fall_dist;
                }
            }

            // Update space
            for p in brick.iter() {
                space.insert((p.x(), p.y()), (p.z(), *id));
            }
        }

        // Now solve part 2
        let mut total_falling = 0;
        for (id, _) in &bricks {
            let mut falling = HashSet::new();
            falling.insert(*id);
            let mut queue: Vec<u32> = supports.get(id).cloned().unwrap_or_default().into_iter().collect();

            while let Some(brick_to_check) = queue.pop() {
                if supported_by.get(&brick_to_check).map_or(false, |s| s.is_subset(&falling)) {
                    if falling.insert(brick_to_check) {
                        if let Some(supported_by_brick) = supports.get(&brick_to_check) {
                            queue.extend(supported_by_brick);
                        }
                    }
                }
            }
            total_falling += falling.len() - 1;
        }
        total_falling as u64
    }
}
