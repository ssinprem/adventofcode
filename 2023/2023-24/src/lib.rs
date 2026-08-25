use regex::*;

pub fn parse(file: String) -> Vec<(u64, u64, u64, i128, i128, i128)> {
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
                && let Ok(dx) = dx.as_str().parse::<i128>()
                && let Some(dy) = cap.get(5)
                && let Ok(dy) = dy.as_str().parse::<i128>()
                && let Some(dz) = cap.get(6)
                && let Ok(dz) = dz.as_str().parse::<i128>()
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
    use super::*;

    pub fn solve(file: String) -> u64 {
        let hails = parse(file);

        let (x0, y0, z0, dx0, dy0, dz0) = hails[0];
        let (x1, y1, z1, dx1, dy1, dz1) = hails[1];
        let (x2, y2, _z2, dx2, dy2, _dz2) = hails[2];
        let x0 = x0 as i128;
        let y0 = y0 as i128;
        let z0 = z0 as i128;
        let x1 = x1 as i128;
        let y1 = y1 as i128;
        let z1 = z1 as i128;
        let x2 = x2 as i128;
        let y2 = y2 as i128;
        for dx in -1000..=1000 {
            for dy in -1000..=1000 {
                let v0x = dx0 - dx;
                let v0y = dy0 - dy;
                let v1x = dx1 - dx;
                let v1y = dy1 - dy;

                let det = v0x * v1y - v0y * v1x;
                if det == 0 {
                    continue;
                }

                let num0 = (x1 - x0) * v1y - (y1 - y0) * v1x;
                let num1 = (x1 - x0) * v0y - (y1 - y0) * v0x;

                if num0 % det != 0 || num1 % det != 0 {
                    continue;
                }

                let t0 = num0 / det;
                let t1 = num1 / det;

                if t0 < 0 || t1 < 0 || t0 == t1 {
                    continue;
                }

                let px = x0 + t0 * v0x;
                let py = y0 + t0 * v0y;

                let v2x = dx2 - dx;
                if v2x == 0 || (px - x2) % v2x != 0 {
                    continue;
                }
                let t2 = (px - x2) / v2x;
                let py_check = y2 + t2 * (dy2 - dy);

                if py == py_check {
                    let cz0 = z0 + t0 * dz0;
                    let cz1 = z1 + t1 * dz1;

                    if (cz1 - cz0) % (t1 - t0) != 0 {
                        continue;
                    }

                    let vz = (cz1-cz0) / (t1 - t0);
                    let pz = cz0 - t0 * vz;

                    println!("{} {} {}", px, py, pz);
                    return (px+py+pz) as u64;
                }
            }
        }

        0
    }
}
