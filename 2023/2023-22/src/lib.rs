use regex::Regex;
use rayon::prelude::*;
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

    pub fn have_upper(top_brick: &Vec<Point3i>, other: &Vec<(u32, Vec<Point3i>)>) -> Vec<u32> {
        other
            .par_iter()
            .filter_map(|(t, ab)| {
                if top_brick.iter().any(|b| {
                    if ab
                        .iter()
                        .any(|a| b.x() == a.x() && b.y() == a.y() && b.z() + 1 == a.z())
                    {
                        true
                    } else {
                        false
                    }
                }) {
                    Some(*t)
                } else {
                    None
                }
            })
            .collect::<Vec<u32>>()
    }

    pub fn have_under(bottom_brick: &Vec<Point3i>, other: &Vec<(u32, Vec<Point3i>)>) -> Vec<u32> {
        other
            .par_iter()
            .filter_map(|(t, ab)| {
                if bottom_brick.iter().any(|b| {
                    if ab
                        .iter()
                        .any(|a| b.x() == a.x() && b.y() == a.y() && b.z() == a.z() + 1)
                    {
                        true
                    } else {
                        false
                    }
                }) {
                    Some(*t)
                } else {
                    None
                }
            })
            .collect::<Vec<u32>>()
    }

    pub fn solve(file: String) -> u64 {
        let mut bricks = parse(file);

        //let failling
        loop {
            let bc = bricks.clone();
            let mut all_done = true;
            bricks.iter_mut().for_each(|(n, b)| {
                let mut grouped: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
                b.iter().for_each(|p| {
                    grouped
                        .entry((p.x(), p.y()))
                        .or_default()
                        .push(p.z() as i32);
                });

                let other = bc
                    .iter()
                    .filter(|(m, _brick)| n != *m)
                    .map(|(m, b)| (*m, b.clone()))
                    .collect::<Vec<(u32, Vec<Point3i>)>>();
                let bottom_brick = grouped
                    .iter()
                    .map(|(xy, z)| PointN([xy.0, xy.1, *z.iter().min().unwrap()]))
                    .collect::<Vec<_>>();

                let have_under = have_under(&bottom_brick, &other);
                let touch_ground = bottom_brick.iter().any(|c| c.z() == 1);

                if !(have_under.len() > 0 || touch_ground) {
                    all_done = false;
                    b.iter_mut().for_each(|c| {
                        let z = c.z_mut();
                        *z -= 1;
                    });
                }
            });

            if all_done {
                break;
            }
        }

        //check each brick
        let bc = bricks.clone();
        let set = bricks
            .par_iter()
            .map(|(n, brick)| {
                let other = bc
                    .iter()
                    .filter(|(m, _brick)| n != *m)
                    .map(|(m, b)| (*m, b.clone()))
                    .collect::<Vec<(u32, Vec<Point3i>)>>();

                let mut grouped: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
                brick.iter().for_each(|p| {
                    grouped
                        .entry((p.x(), p.y()))
                        .or_default()
                        .push(p.z() as i32);
                });

                let top_brick = grouped
                    .iter()
                    .map(|(xy, z)| PointN([xy.0, xy.1, *z.iter().max().unwrap()]))
                    .collect::<Vec<_>>();

                let bottom_brick = grouped
                    .iter()
                    .map(|(xy, z)| PointN([xy.0, xy.1, *z.iter().min().unwrap()]))
                    .collect::<Vec<_>>();

                let have_upper = have_upper(&top_brick, &other);
                let have_under = have_under(&bottom_brick, &other);

                // println!("{n} {have_on:?} {have_under:?}");
                (*n, (have_upper, have_under))
            })
            .collect::<HashMap<u32, (Vec<u32>, Vec<u32>)>>();

        let keys = set.keys().into_iter().copied().collect::<Vec<u32>>();
        keys.par_iter()
            .filter(|&key| {
                let s = set.get(key).unwrap();
                let uppers = s.0.clone();

                uppers.is_empty()
                    || uppers.iter().all(|upper| {
                        let u = set.get(upper).unwrap();
                        let unders = u.1.clone();
                        unders.len() > 1
                    })
            })
            .count() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
