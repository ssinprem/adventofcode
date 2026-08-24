use regex::*;

pub fn parse(file: String) -> Vec<(u64, u64, u64, i64, i64, i64)> {
    let regex =
        Regex::new(r"([0-9]+),\s*([0-9]+),\s*([0-9]+)\s*@\s*([-0-9]+),\s*([-0-9]+),\s*([-0-9]+)")
            .unwrap();
    file.lines()
        .filter_map(|line| {
            if let Some(cap) = regex.captures(line)
                && let Some(x) = cap.get(1)
                && let Ok(x) = x.as_str().parse::<u64>()
                && let Some(y) = cap.get(2)
                && let Ok(y) = y.as_str().parse::<u64>()
                && let Some(z) = cap.get(3)
                && let Ok(z) = z.as_str().parse::<u64>()
                && let Some(dx) = cap.get(4)
                && let Ok(dx) = dx.as_str().parse::<i64>()
                && let Some(dy) = cap.get(5)
                && let Ok(dy) = dy.as_str().parse::<i64>()
                && let Some(dz) = cap.get(6)
                && let Ok(dz) = dz.as_str().parse::<i64>()
            {
                Some((x, y, z, dx, dy, dz))
            } else {
                None
            }
        })
        .collect()
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String, min: u64, max: u64) -> u64 {
        let hails = parse(file);
        let lines: Vec<_> = hails
            .iter()
            .map(|(x, y, _, dx, dy, _)| {
                let m = *dy as f64 / *dx as f64;
                let c = *y as f64 - (m * (*x) as f64);
                (*x, *y, *dx, *dy, m, c)
            })
            .collect();

        let mut cnt = 0;
        for i in 0..lines.len() {
            for j in i + 1..lines.len() {
                let (x1, _y1, dx1, _dy1, m1, c1) = lines[i];
                let (x2, _y2, dx2, _dy2, m2, c2) = lines[j];
                if m1 == m2 {
                    continue;
                }

                let x = (c2 - c1) / (m1 - m2);
                let y = m1 * x + c1;
                if x < min as f64
                    || x > max as f64
                    || y < min as f64
                    || y > max as f64
                    || (x - x1 as f64) / (dx1 as f64) < 0.0
                    || (x - x2 as f64) / (dx2 as f64) < 0.0
                {
                    continue;
                }
                cnt += 1;
            }
        }

        cnt
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
