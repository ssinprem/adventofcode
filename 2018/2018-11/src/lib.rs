pub fn fuel_power(x: usize, y: usize, serial: usize) -> i32 {
    let rack_id = x + 10;
    let tmp = rack_id * y;
    let tmp = tmp + serial;

    let tmp = tmp * rack_id;
    let tmp = (tmp / 100) % 10;
    tmp as i32 - 5
}

pub fn generate_grid(serial: usize) -> [[i32; 301]; 301] {
    let mut grid = [[0; 301]; 301];

    for y in 1..=300 {
        for x in 1..=300 {
            grid[x][y] = fuel_power(x, y, serial);
        }
    }

    grid
}

pub mod part1 {
    use crate::generate_grid;

    pub fn solve(file: String) -> (i32, (usize, usize)) {
        let serial = file.parse::<usize>().expect("cannot parse serial");
        let grid = generate_grid(serial);

        (1..298)
            .map(|y| {
                (1..298)
                    .map(|x| {
                        let sum = (0..3)
                            .map(|dy| (0..3).map(|dx| grid[x + dx][y + dy]).sum::<i32>())
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
    use crate::generate_grid;
    use rayon::prelude::*;

    pub fn solve(file: String) -> (i32, (usize, usize, usize)) {
        let serial = file.parse::<usize>().expect("cannot parse serial");

        let grid = generate_grid(serial);

        (1..300)
            .into_par_iter()
            .map(|size| {
                let mut best = (i32::MIN, (0, 0, size));
                for y in 1..=(301 - size) {
                    for x in 1..=(301 - size) {
                        let sum = (0..size)
                            .map(|dy| (0..size).map(|dx| grid[x + dx][y + dy]).sum::<i32>())
                            .sum::<i32>();
                        if sum > best.0 {
                            best = (sum, (x, y, size));
                        }
                    }
                }
                best
            })
            .max()
            .unwrap()
    }
}
