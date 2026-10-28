
pub fn fuel_power(x: isize, y: isize, serial: isize) -> i32 {
    let rack_id = x + 10;
    let tmp = rack_id * y;
    let tmp = tmp + serial;

    let tmp = tmp * rack_id;
    let tmp = (tmp / 100) % 10;
    tmp as i32 - 5
}

pub mod part1 {
    use std::collections::HashMap;

use crate::fuel_power;

    pub fn solve(file: String) -> (i32, (isize,isize)) {
        let serial = file.parse::<isize>().expect("cannot parse serial");

        let mut grid = HashMap::new();

        for y in 1..=300_isize {
            for x in 1..=300_isize {
                grid.insert((x,y), fuel_power(x, y, serial));
            }
        }

        (2..299).map(|y: isize| {
            (2..299).map(|x| {
                let sum = [(-1,-1),(0,-1),(1,-1),
                (-1,0),(0,0),(1,0),
                (-1,1),(0,1),(1,1)].map(|(dx,dy)| {
                    *grid.get(&(x+dx,y+dy)).expect("cannot get grid value")
                }).iter().sum();

                (sum, (x-1,y-1))
            }).max().unwrap()
        }).max().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
