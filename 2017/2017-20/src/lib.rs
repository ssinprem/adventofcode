pub fn parse(file: String) -> Vec<[i64; 9]> {
    use regex::Regex;
    let regex = Regex::new(r"p=<([-0-9]+),([-0-9]+),([-0-9]+)>, v=<([-0-9]+),([-0-9]+),([-0-9]+)>, a=<([-0-9]+),([-0-9]+),([-0-9]+)>").unwrap();

    file.lines()
        .filter_map(|line| {
            if let Some(matched) = regex.captures(line)
                && let Some(px) = matched.get(1)
                && let Ok(px) = px.as_str().parse::<i64>()
                && let Some(py) = matched.get(2)
                && let Ok(py) = py.as_str().parse::<i64>()
                && let Some(pz) = matched.get(3)
                && let Ok(pz) = pz.as_str().parse::<i64>()
                && let Some(vx) = matched.get(4)
                && let Ok(vx) = vx.as_str().parse::<i64>()
                && let Some(vy) = matched.get(5)
                && let Ok(vy) = vy.as_str().parse::<i64>()
                && let Some(vz) = matched.get(6)
                && let Ok(vz) = vz.as_str().parse::<i64>()
                && let Some(ax) = matched.get(7)
                && let Ok(ax) = ax.as_str().parse::<i64>()
                && let Some(ay) = matched.get(8)
                && let Ok(ay) = ay.as_str().parse::<i64>()
                && let Some(az) = matched.get(9)
                && let Ok(az) = az.as_str().parse::<i64>()
            {
                Some([px, py, pz, vx, vy, vz, ax, ay, az])
            } else {
                None
            }
        })
        .collect()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> usize {
        let list = parse(file);

        list.iter()
            .enumerate()
            .min_by_key(|(_id, [_px, _py, _pz, _vx, _vy, _vz, ax, ay, az])| {
                ax.abs() + ay.abs() + az.abs()
            })
            .unwrap()
            .0
    }
}

pub mod part2 {
    use crate::parse;
    use itertools::Itertools;
    use std::collections::HashSet;

    pub fn solve(file: String) -> usize {
        let mut list = parse(file).into_iter().enumerate().collect::<Vec<_>>();

        for _i in 0..100 {
            for (_id, [px, py, pz, vx, vy, vz, ax, ay, az]) in list.iter_mut() {
                *vx += *ax;
                *vy += *ay;
                *vz += *az;
                *px += *vx;
                *py += *vy;
                *pz += *vz;
            }

            let mut collision = HashSet::new();
            let temp = list.clone();
            temp.iter().permutations(2).for_each(|pair| {
                let (ida, [pxa, pya, pza, _, _, _, _, _, _]) = pair[0];
                let (idb, [pxb, pyb, pzb, _, _, _, _, _, _]) = pair[1];

                if pxa == pxb && pya == pyb && pza == pzb {
                    collision.insert(ida);
                    collision.insert(idb);
                }
            });
            for c in collision {
                if let Some(pos) = list.iter().position(|(id, _)| id == c) {
                    list.remove(pos);
                }
            }
        }

        list.len()
    }
}
