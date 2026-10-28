pub fn fuel_power(x: isize, y: isize, serial: isize) -> i32 {
    let rack_id = x + 10;
    let tmp = rack_id * y;
    let tmp = tmp + serial;

    let tmp = tmp * rack_id;
    let tmp = (tmp / 100) % 10;
    tmp as i32 - 5
}

pub mod part1 {
    use crate::fuel_power;
    use std::collections::HashMap;

    pub fn solve(file: String) -> (i32, (isize, isize)) {
        let serial = file.parse::<isize>().expect("cannot parse serial");

        let mut grid = HashMap::new();

        for y in 1..=300_isize {
            for x in 1..=300_isize {
                grid.insert((x, y), fuel_power(x, y, serial));
            }
        }

        (1..298)
            .map(|y: isize| {
                (1..298)
                    .map(|x| {
                        let sum = (0..3)
                            .map(|dy| {
                                (0..3)
                                    .map(|dx| {
                                        *grid.get(&(x + dx, y + dy)).expect("cannot get grid value")
                                    })
                                    .sum::<i32>()
                            })
                            .sum();

                        (sum, (x, y))
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

pub mod part2 {
    use crate::fuel_power;
    use rayon::prelude::*;
    use std::collections::HashMap;

    pub fn solve(file: String) -> (i32, (isize, isize, isize)) {
        let serial = file.parse::<isize>().expect("cannot parse serial");

        let mut grid = HashMap::new();

        for y in 1..=300_isize {
            for x in 1..=300_isize {
                grid.insert((x, y), fuel_power(x, y, serial));
            }
        }

        (1..300)
            .into_par_iter()
            .map(|y| {
                (1..300)
                    .into_par_iter()
                    .map(|x| {
                        let max_size = 300 - x.max(y);
                        // limit 20x20 for optimize
                        let max_size = 20.min(max_size); 
                        (0..max_size)
                            .into_par_iter()
                            .map(|size| {
                                let sum: i32 = (0..size)
                                    .into_par_iter()
                                    .map(|dy| {
                                        (0..size)
                                            .into_par_iter()
                                            .map(|dx| {
                                                *grid
                                                    .get(&(x + dx, y + dy))
                                                    .expect("cannot get grid value")
                                            })
                                            .sum::<i32>()
                                    })
                                    .sum();
                                // println!("{x},{y},{size},{sum}");
                                (sum, (x, y, size))
                            })
                            .max()
                            .unwrap()
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}
